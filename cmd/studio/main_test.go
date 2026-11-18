package main

import (
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestGeneratedLoader(t *testing.T) {
	dir := t.TempDir()
	if e := generate(dir); e != nil {
		t.Fatal(e)
	}
	b, e := os.ReadFile(filepath.Join(dir, "index.html"))
	if e != nil {
		t.Fatal(e)
	}
	s := string(b)
	for _, v := range []string{title, "proposal", "type=\"module\"", "./pkg/saskatchewan_library_funding.js"} {
		if !strings.Contains(s, v) {
			t.Fatalf("Missing %s", v)
		}
	}
	if strings.Contains(s, "<style") || strings.Contains(s, "stylesheet") {
		t.Fatal("Unexpected authored CSS")
	}
}
func TestReadOnlyServer(t *testing.T) {
	dir := t.TempDir()
	generate(dir)
	h := handler(dir)
	for _, method := range []string{"POST", "PUT", "DELETE"} {
		r := httptest.NewRequest(method, "/", nil)
		w := httptest.NewRecorder()
		h.ServeHTTP(w, r)
		if w.Code != 405 {
			t.Fatal(w.Code)
		}
	}
	w := httptest.NewRecorder()
	h.ServeHTTP(w, httptest.NewRequest("GET", "/", nil))
	if w.Code != 200 {
		t.Fatal(w.Code)
	}
	if !strings.Contains(w.Body.String(), title) {
		t.Fatal("Missing page")
	}
}
