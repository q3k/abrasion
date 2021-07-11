package main

import (
	"archive/zip"
	"crypto/sha256"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"sort"

	"google.golang.org/protobuf/encoding/prototext"
	"google.golang.org/protobuf/proto"

	pb "abrasion/tools/release/proto"
)

var (
	flagManifest string
	flagExe      string
	flagZip      string
)

func packFile(w *zip.Writer, file *pb.File) error {
	fo, err := w.Create(file.ShortPath)
	if err != nil {
		return fmt.Errorf("Create: %w", err)
	}
	// TODO(q3k): maybe don't read this into memory...
	data, err := ioutil.ReadFile(file.Path)
	if err != nil {
		return fmt.Errorf("Open: %w", err)
	}
	h := sha256.Sum256(data)
	file.Sha256 = h[:]
	_, err = fo.Write(data)
	if err != nil {
		return fmt.Errorf("Write: %w", err)
	}
	// We don't need this in the release manifest.
	file.Path = ""
	return nil
}

func main() {
	flag.StringVar(&flagManifest, "pack_manifest", "", "Path to manifest.text.pb")
	flag.StringVar(&flagExe, "pack_exe", "", "Path to main .exe")
	flag.StringVar(&flagZip, "pack_zip", "", "Path to generated release .zip")
	flag.Parse()

	data, err := ioutil.ReadFile(flagManifest)
	if err != nil {
		log.Fatalf("ReadFile: %v", err)
	}
	var manifest pb.Manifest
	if err := prototext.Unmarshal(data, &manifest); err != nil {
		log.Fatalf("Unmashal: %v", err)
	}

	sort.Slice(manifest.File, func(i, j int) bool {
		return manifest.File[i].ShortPath < manifest.File[j].ShortPath
	})

	f, err := os.Create(flagZip)
	if err != nil {
		log.Fatalf("Create: %v", err)
	}
	defer f.Close()

	w := zip.NewWriter(f)
	defer w.Close()

	// Pack runfiles
	for _, file := range manifest.File {
		if err := packFile(w, file); err != nil {
			log.Fatalf("Failed to pack file %q (%q): %v", file.ShortPath, file.Path, err)
		}
	}

	// Pack engine
	engine := pb.File{
		ShortPath: "abrasion.exe",
		Path:      flagExe,
	}
	if err := packFile(w, &engine); err != nil {
		log.Fatalf("Failed to pack engine: %v", err)
	}
	manifest.File = append(manifest.File, &engine)

	// Pack binary manifest.
	manifestBytes, err := proto.Marshal(&manifest)
	if err != nil {
	}
	mo, err := w.Create("abrasion.manifest")
	if err != nil {
		log.Fatalf("Failed to create manifest: %v", err)
	}
	if _, err := mo.Write(manifestBytes); err != nil {
		log.Fatalf("Failed to write manifest: %v", err)
	}
}
