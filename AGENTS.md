# CRITICAL RULES - MUST FOLLOW

## RESPONSES

- Keep responses concise and to the point - unless the user asks otherwise

## PLANNING MODE

- Always ask clarifying questions
- Never assume design, tech stack or features

## CHANGE / EDIT MODE

- After completing features (large or small), always run commands like lint, type check and next build to check code
  quality and format the code with `mise format`

## TESTING

- Use any testing tools, libraries available to the project for testing your changes
- Never assume your changes simply work, always test!

## PARALLELIZATION

- Always launch independent tool calls in parallel within the same message
- Never wait for one tool's result before calling another tool that doesn't depend on it
- Limit to 3 Explore agents max per task to balance speed and coverage

## SUBAGENTS

- File search, grep, code location → spawn `caveman:cavecrew-investigator` with `model: "haiku"` — never use Bash
  grep/find/cat in main thread for exploration
- 1-2 file edits (typo, rename, single function) → spawn `caveman:cavecrew-builder` with `model: "haiku"`
- Diff/PR review → spawn `caveman:cavecrew-reviewer` with `model: "haiku"`
- Use `Read` directly only when the exact file path and line range are already known
- Use `Edit`/`Write` directly in main thread for code changes after research is done

## PLAYWRIGHT

- Save your screen captures and logs from playwright in the `.playwright-mcp` folder at the root of the repository

## Instructions

- **Authentication**: [authentication.instructions.md](.agents/authentication.instructions.md)
- **Backend**: [backend.instructions.md](.agents/backend.instructions.md)
- **CI/CD**: [ci.instructions.md](.agents/ci.instructions.md)
- **Database Schema**: [database-schema.instructions.md](.agents/database-schema.instructions.md)
- **API Endpoints**: [endpoints.instructions.md](.agents/endpoints.instructions.md)
- **Mise & Workflow**: [mise.instructions.md](.agents/mise.instructions.md)