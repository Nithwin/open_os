# Contributing to OpenOS

Thank you for your interest in contributing to OpenOS! We believe that this project is better when built together. Whether you're a student learning OS concepts, an educator looking to improve the codebase, or an experienced developer, your contributions are valuable.

## Ways to Contribute

### 1. **Improve Documentation**
- Clarify confusing explanations
- Add inline comments to complex sections
- Write blog posts or tutorials about specific topics
- Fix typos or grammar issues

### 2. **Fix Bugs**
- Report issues with clear examples
- Submit pull requests with bug fixes
- Test on different systems and report findings

### 3. **Add Educational Features**
- Implement new OS concepts (interrupts, memory management, file systems, etc.)
- Add keyboard input handling
- Implement basic multitasking
- Create diagnostic tools

### 4. **Improve Code Quality**
- Refactor code for clarity
- Add more comprehensive examples
- Optimize performance where it helps learning
- Add tests and validation code

### 5. **Share Knowledge**
- Open issues with questions or discussions
- Help answer other learners' questions
- Contribute examples or experiments
- Suggest learning resources

## Getting Started

### Fork & Clone
```bash
# Fork the repository on GitHub, then:
git clone https://github.com/yourusername/open_os.git
cd open_os
```

### Make Changes
1. Create a new branch for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes with clear, documented code
3. Test thoroughly:
   ```bash
   cargo build
   cargo run
   ```

### Commit & Push
```bash
git add .
git commit -m "Clear description of your changes"
git push origin feature/your-feature-name
```

### Submit a Pull Request
- Go to the original repository on GitHub
- Click "New Pull Request"
- Describe what you changed and why
- Link any related issues

## Contribution Guidelines

### Code Style
- **Comment liberally**: Every function should explain what it does and why
- **Use clear names**: Variable and function names should be self-documenting
- **Follow Rust conventions**: Use `cargo fmt` to format code consistently
- **Explain the "why"**: Don't just explain the "what"—help readers understand the reasoning

### Documentation
- Add doc comments (`///`) to all public items
- Include examples in documentation where helpful
- Update README.md if you add major features
- Keep explanations beginner-friendly

### Commits
- Write descriptive commit messages
- One logical change per commit (don't mix unrelated changes)
- Reference issues when relevant: "Fixes #42" in commit message

### Pull Request Process
1. **Describe your change**: What problem does it solve? What did you learn?
2. **Include context**: Why is this change important?
3. **Be respectful**: We're all learning here
4. **Be patient**: Reviews take time, and maintainers may have questions

## Reporting Issues

Found a bug or have a suggestion? Open an issue with:
- **Clear title**: What's the problem?
- **Description**: What did you expect vs. what happened?
- **Steps to reproduce**: How can we reproduce the issue?
- **Environment**: OS, Rust version, etc.

Example:
```
Title: Scrolling doesn't clear the last row properly

Description:
When the screen is full and new text is added, the last row isn't cleared
before new content appears.

Steps:
1. Boot OpenOS
2. Print enough text to fill 25 rows
3. Print one more line

Expected: New line appears on empty row 24
Actual: Old content from row 24 still visible

Environment: Ubuntu 22.04, Rust 1.75, QEMU
```

## Learning & Discussion

Have questions about OS concepts? Want to discuss ideas?
- Open an issue with "Discussion:" prefix
- Share what you've learned
- Ask for clarification on confusing parts
- Suggest improvements to explanations

This is a community for learning, not just code submission!

## Code of Conduct

OpenOS is committed to being an inclusive, welcoming community. We expect:
- **Respect**: Treat everyone with kindness
- **Patience**: We're all learning at different levels
- **Openness**: Embrace diverse perspectives and ideas
- **Constructiveness**: Offer feedback that helps, not hurts

Any harassment, discrimination, or disrespect will not be tolerated.

## Recognition

Contributors are valued! We'll recognize all contributions:
- Mention in README.md's contributor section
- Credit in commit history
- Community appreciation for significant contributions

## Questions?

Not sure where to start? That's okay!
- Look for "good first issue" labels
- Read the code comments carefully
- Ask questions in issues
- Start small and build up

## Legal

By contributing to OpenOS, you agree that your contributions will be licensed under the MIT License. You confirm that you have the right to submit the work and that you're not violating any copyrights.

---

**Thank you for making OpenOS better for everyone! 🙌**

We're excited to learn alongside you. Let's build the best OS learning resource on GitHub together!
