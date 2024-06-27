package main

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"panel/config"
	"path/filepath"
	"strings"

	"github.com/wailsapp/wails/v2/pkg/runtime"
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

	Config *config.Config
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

		// if the directory does not contain a file named "panel.yml" then skip it
		if _, err := os.Stat(filepath.Join(root, path, "panel.yml")); os.IsNotExist(err) {
			return nil
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

func (a *App) LoadProject(path string) error {
	var err error

	filename := filepath.Join(path, "panel.yml")
	fmt.Println("Load Project: ", filename)

	a.Config, err = config.GetConfig(filename)
	if err != nil {
		fmt.Println("Error loading config: ", err)
		return err
	}

	runtime.WindowSetTitle(a.ctx, a.Config.App.Title)

	return nil
}
