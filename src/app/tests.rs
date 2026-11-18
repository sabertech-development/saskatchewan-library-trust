use super::*;
fn setup() -> (Context, ProposalApp) {
    let ctx = Context::default();
    ctx.enable_accesskit();
    let app = ProposalApp::with_context(&ctx);
    (ctx, app)
}
fn tick(app: &mut ProposalApp, ctx: &Context, w: f32, events: Vec<Event>) -> FullOutput {
    ctx.run(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(w, 1200.))),
            events,
            ..Default::default()
        },
        |ctx| app.render(ctx),
    )
}
fn click(app: &mut ProposalApp, ctx: &Context, label: &str, w: f32) {
    let mut out = tick(app, ctx, w, vec![]);
    for _ in 0..2 {
        out = tick(app, ctx, w, vec![]);
    }
    let tree = out.platform_output.accesskit_update.expect("tree");
    let b = tree
        .nodes
        .iter()
        .find_map(|(_, n)| {
            if n.label() == Some(label) || n.value() == Some(label) {
                n.bounds()
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Missing control {label}"));
    assert!(
        b.x0 >= 0. && b.x1 <= w as f64 + 1.,
        "Control outside viewport: {label}"
    );
    let pos = pos2(((b.x0 + b.x1) / 2.) as f32, ((b.y0 + b.y1) / 2.) as f32);
    tick(app, ctx, w, vec![Event::PointerMoved(pos)]);
    for pressed in [true, false] {
        tick(
            app,
            ctx,
            w,
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
    }
}
#[test]
fn all_routes_render_at_mobile_tablet_and_desktop_sizes() {
    for w in [390., 820., 1440.] {
        let (ctx, mut app) = setup();
        for route in 0..ROUTES.len() {
            app.navigate(route);
            for _ in 0..3 {
                let output = tick(&mut app, &ctx, w, vec![]);
                assert!(output.shapes.len() > 20);
            }
        }
    }
}
#[test]
fn navigation_and_five_question_tour_work() {
    let (ctx, mut app) = setup();
    for (i, (_, name)) in ROUTES.iter().enumerate() {
        click(&mut app, &ctx, name, 1440.);
        assert_eq!(app.route, i);
    }
    app.navigate(0);
    click(&mut app, &ctx, "Take the guided tour", 1440.);
    for step in 1..TOUR.len() {
        click(&mut app, &ctx, "Next stop", 1440.);
        assert_eq!(app.route, TOUR[step].0);
    }
    click(&mut app, &ctx, "Finish in review summary", 1440.);
    assert_eq!(app.route, 9);
    assert!(app.tour.is_none());
}
#[test]
fn mobile_menu_keyboard_and_navigation_work() {
    let (ctx, mut app) = setup();
    click(&mut app, &ctx, "Menu", 390.);
    assert!(app.mobile_menu);
    click(&mut app, &ctx, "Library Atlas", 390.);
    assert_eq!(app.route, 1);
    assert!(!app.mobile_menu);
    click(&mut app, &ctx, "Menu", 390.);
    tick(
        &mut app,
        &ctx,
        390.,
        vec![Event::Key {
            key: Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
    );
    assert!(!app.mobile_menu);
    tick(
        &mut app,
        &ctx,
        390.,
        vec![Event::Key {
            key: Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
    );
    assert!(ctx.memory(|m| m.focused()).is_some());
}
#[test]
fn scenario_presets_drive_shared_projection() {
    let (ctx, mut app) = setup();
    app.navigate(4);
    click(
        &mut app,
        &ctx,
        "Full-target hypothetical illustration",
        1440.,
    );
    click(&mut app, &ctx, "Continuing losses", 1440.);
    assert!(app.inputs.return_rate < 0.);
    let p = finance::calculate(&app.inputs).unwrap();
    assert_eq!(p.years[0].endowment_distribution, 0.);
    click(&mut app, &ctx, "Reset hypothetical inputs", 1440.);
    assert!(app.inputs.interest_only);
    assert_eq!(app.inputs.capital, PRINCIPAL_TARGET);
}
#[test]
fn atlas_selection_tabs_and_zoom_work() {
    let (ctx, mut app) = setup();
    app.navigate(1);
    click(&mut app, &ctx, "Select Corporate donors", 1440.);
    assert_eq!(app.node, 0);
    click(&mut app, &ctx, "Relationships", 1440.);
    assert_eq!(app.inspector_tab, 1);
    click(&mut app, &ctx, "+", 1440.);
    assert!(app.map_zoom > 1.);
    click(&mut app, &ctx, "Reset", 1440.);
    assert_eq!(app.map_zoom, 1.);
}
#[test]
fn system_light_mode_does_not_override_dark_presentation() {
    let (ctx, mut app) = setup();
    let _ = ctx.run(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1440., 900.))),
            system_theme: Some(Theme::Light),
            ..Default::default()
        },
        |ctx| app.render(ctx),
    );
    assert!(ctx.style().visuals.dark_mode);
    assert_eq!(ctx.style().visuals.widgets.inactive.weak_bg_fill, PANEL);
}

#[test]
fn primary_review_views_fit_a_laptop_without_content_scrolling() {
    let (ctx, mut app) = setup();
    let mut failures = Vec::new();
    for route in [0, 2, 3, 4, 5, 6, 7, 8, 9] {
        app.navigate(route);
        for _ in 0..4 {
            let _ = ctx.run(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1366., 768.))),
                    ..Default::default()
                },
                |ctx| app.render(ctx),
            );
        }
        let overflow = ctx.data(|d| d.get_temp::<Vec<f32>>(Id::new("panel-overflows")).unwrap());
        if overflow.iter().any(|&v| v > 1.) {
            failures.push(format!("Route {route}: {overflow:?}"));
        }
    }
    app.navigate(3);
    app.funding_tab = 1;
    for _ in 0..4 {
        let _ = ctx.run(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1366., 768.))),
                ..Default::default()
            },
            |ctx| app.render(ctx),
        );
    }
    let overflow = ctx.data(|d| d.get_temp::<Vec<f32>>(Id::new("panel-overflows")).unwrap());
    if overflow.iter().any(|&v| v > 1.) {
        failures.push(format!("Funding calculator: {overflow:?}"));
    }
    assert!(
        failures.is_empty(),
        "Views requiring scroll: {}",
        failures.join(", ")
    );
}

#[test]
fn funding_and_scenario_tabs_replace_the_selected_view() {
    let (ctx, mut app) = setup();
    app.navigate(3);
    click(&mut app, &ctx, "02  Scenario calculator", 1440.);
    assert_eq!(app.funding_tab, 1);
    click(&mut app, &ctx, "Returns", 1440.);
    assert_eq!(app.input_tab, 1);
    click(&mut app, &ctx, "Fund & value", 1440.);
    assert_eq!(app.result_tab, 1);
    click(&mut app, &ctx, "03  Funding & accountability", 1440.);
    assert_eq!(app.funding_tab, 2);
    app.navigate(0);
    click(&mut app, &ctx, "Service outcomes", 1440.);
    assert_eq!(app.review_tabs[0], 2);
}
