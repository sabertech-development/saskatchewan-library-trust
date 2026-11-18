// Studio builds and serves the Rust browser application. No financial inputs
// are sent here: the server only serves the immutable build output.
package main

import (
	"flag"
	"fmt"
	"log"
	"mime"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"
)

const title = "Saskatchewan Library Sustainability Trust"

// The visual canvas and calculation engine are Rust. A semantic HTML companion
// uses the same WASM engine with native form controls and a small DOM adapter.
const loader = `<!doctype html>
<html lang="en-CA"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="theme-color" content="#030509"><meta name="color-scheme" content="dark"><meta name="description" content="A proposed donation-funded Saskatchewan Library Sustainability Trust. Hypothetical scenarios. No donations collected."><title>` + title + `</title><style>
@font-face{font-family:Space;src:url('fonts/SpaceGrotesk.ttf') format('truetype');font-display:swap}*{box-sizing:border-box}body{margin:0;background:#030509;color:#e8f2fa;font:16px Space,system-ui,sans-serif}#proposal{position:fixed;inset:0;width:100%;height:100%;display:block;opacity:0}.skip{position:fixed;top:0;left:20px;z-index:10;padding:12px;background:#08111d;color:#b4edff;transform:translateY(-150%)}.skip:focus{transform:translateY(0);outline:3px solid #3dcaff}#loading{position:relative;z-index:2;max-width:880px;margin:10vh auto;padding:36px;border:1px solid #254456;background:#091622}#loading h1{font-size:clamp(27px,4vw,46px);line-height:1.14;margin:18px 0;color:#e8f2fa}#loading p{line-height:1.6;color:#a8bece}.eyebrow{color:#3dcaff!important;font-size:12px;letter-spacing:2px}#loading-status{color:#b4edff!important}#loading a{display:inline-block;padding:13px 18px;margin:8px 10px 0 0;border:1px solid #3dcaff;color:#b4edff;text-decoration:none}#loading a:focus-visible{outline:3px solid #b4edff;outline-offset:3px}@media(max-width:620px){#loading{margin:6vh 16px;padding:24px}}
</style></head><body><a class="skip" href="reader.html">Accessible reading and scenario calculator</a><canvas id="proposal" tabindex="0" aria-label="Saskatchewan Library Sustainability Trust visual presentation. Use Tab and Enter for controls. An accessible reading and scenario calculator link is available before this canvas."></canvas>
<main id="loading"><p class="eyebrow">PREPARED BY SABER TECH INC.</p><h1>Saskatchewan Library Sustainability Trust</h1><p>Building a lasting fund for Saskatchewan libraries through donations, responsible investment, and transparent community support.</p><p id="loading-status" role="status" aria-live="polite">Opening the proposal...</p><a href="reader.html">Open accessible proposal</a><a href="library-trust-review.pdf">Read the review PDF</a><p><small>Proposed corporate donation target: CA$250 million. Not secured funds. No donations collected.</small></p><noscript><p>Open the accessible proposal above to read the presentation without JavaScript.</p></noscript></main>
<script type="module" src="report-download.js"></script><script type="module" src="site-loader.js"></script></body></html>`

func run(name string, args ...string) error {
	c := exec.Command(name, args...)
	c.Stdout = os.Stdout
	c.Stderr = os.Stderr
	c.Stdin = os.Stdin
	return c.Run()
}
func generate(directory string) error {
	if err := os.MkdirAll(directory, 0755); err != nil {
		return err
	}
	if err := os.WriteFile(filepath.Join(directory, "index.html"), []byte(loader), 0644); err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(directory, ".nojekyll"), nil, 0644)
}
func handler(directory string) http.Handler {
	files := http.FileServer(http.Dir(directory))
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("X-Content-Type-Options", "nosniff")
		w.Header().Set("Referrer-Policy", "no-referrer")
		w.Header().Set("Cache-Control", "no-cache")
		if r.Method != "GET" && r.Method != "HEAD" {
			http.Error(w, "Read-only demonstration", http.StatusMethodNotAllowed)
			return
		}
		if r.URL.Path == "/healthz" {
			w.Header().Set("Content-Type", "text/plain")
			fmt.Fprint(w, "ok")
			return
		}
		files.ServeHTTP(w, r)
	})
}
func main() {
	flag.Usage = func() {
		fmt.Fprintln(os.Stderr, "Usage: go run ./cmd/studio [build|serve|generate|policy] [options]\nAll financial calculations run locally in Rust WebAssembly.")
	}
	if len(os.Args) < 2 {
		flag.Usage()
		os.Exit(2)
	}
	switch os.Args[1] {
	case "build":
		if err := run("wasm-pack", "build", "--release", "--target", "web", "--out-dir", "dist/pkg", "--no-typescript"); err != nil {
			log.Fatal(err)
		}
		if err := generate("dist"); err != nil {
			log.Fatal(err)
		}
		if err := run("cargo", "run", "--example", "export_review"); err != nil {
			log.Fatal(err)
		}
		if err := run("node", "scripts/export_report.mjs"); err != nil {
			log.Fatal(err)
		}
		fmt.Println("Browser build ready in dist/")
	case "generate":
		if err := generate("dist"); err != nil {
			log.Fatal(err)
		}
	case "serve":
		fs := flag.NewFlagSet("serve", flag.ExitOnError)
		address := fs.String("addr", "127.0.0.1:4387", "local listening address")
		directory := fs.String("dir", "dist", "build directory")
		fs.Parse(os.Args[2:])
		if _, err := os.Stat(filepath.Join(*directory, "index.html")); err != nil {
			log.Fatal("Build first: go run ./cmd/studio build")
		}
		mime.AddExtensionType(".wasm", "application/wasm")
		mime.AddExtensionType(".js", "text/javascript")
		mime.AddExtensionType(".mjs", "text/javascript")
		fmt.Printf("Local preview: http://%s/\n", *address)
		server := &http.Server{Addr: *address, Handler: handler(*directory), ReadHeaderTimeout: 5 * time.Second}
		log.Fatal(server.ListenAndServe())
	case "policy":
		command := policyCommand()
		if err := run(command, "transcript", "policy/verify.md"); err != nil {
			log.Fatal(err)
		}
	default:
		flag.Usage()
		os.Exit(2)
	}
}
