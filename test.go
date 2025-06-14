package main

import (
    "fmt"
    tea "github.com/charmbracelet/bubbletea"
    "github.com/charmbracelet/lipgloss"
)

var boxStyle = lipgloss.NewStyle().
    Border(lipgloss.RoundedBorder()).
    Padding(, ).//height,width
    BorderForeground(lipgloss.Color("#FAFAFA")).
    Background(lipgloss.Color("#7D56F4")).
    Foreground(lipgloss.Color("#FFFFFF"))

type model struct {
    count int
}

func (m model) Init() tea.Cmd {
    return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    switch msg := msg.(type) {
    case tea.KeyMsg:
        switch msg.String() {
        case "q":
            return m, tea.Quit
        case "up", "k":
            m.count++
        case "down", "j":
            m.count--
        }
    }
    return m, nil
}

func (m model) View() string {
    content := fmt.Sprintf(
        "Count: %d\n\n操作:\n  ↑ or k: カウントアップ\n  ↓ or j: カウントダウン\n  q: 終了\n",
        m.count,
    )
    return boxStyle.Render(content)
}

func main() {
    p := tea.NewProgram(model{})
    if err := p.Start(); err != nil {
        fmt.Println("Error:", err)
    }
}
