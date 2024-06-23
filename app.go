package main

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// Project struct for loading project data
type Project struct {
	Path         string `json:"path"`
	Description  string `json:"description"`
	ModifiedTime string `json:"modifiedTime"`
}

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

func (a *App) GetProjects() []Project {
	projects := map[string]Project{}

	maxDepth := 0
	root := fmt.Sprintf("%s/Projects", os.Getenv("HOME"))
	fileSystem := os.DirFS(root)

	fs.WalkDir(fileSystem, ".", func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			return nil
		}

		if d.IsDir() && strings.Count(path, string(os.PathSeparator)) > maxDepth {
			return fs.SkipDir
		}

		info, derr := d.Info()
		if derr != nil {
			return derr
		}

		project := Project{
			Path:         filepath.Join(root, path),
			Description:  "This is a description",
			ModifiedTime: info.ModTime().String(),
		}
		projects[path] = project

		return nil
	})

	result := []Project{}
	for _, p := range projects {
		result = append(result, p)
	}

	return result
}
