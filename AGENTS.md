# CRITICAL RULES - MUST FOLLOW

## RESPONSES

- Keep responses concise and to the point - unless the user asks otherwise

## PLANNING MODE

- Always ask clarifying questions
- Never assume design, tech stack or features
- Use deep-dive sub-agents to assist with research
- Use deep-dive sub-agents to review the different aspects of your plan before presenting to the user
- Create a plan with clear steps and deliverables, and save it in the .agents/plans directory for future reference and
  implementation

## CHANGE / EDIT MODE

- Never implement features yourself when possible - use sub-agents!
- Identify changes from the plan that can be implemented in parallel, and use sub-agents to implement the features
  efficiently
- When using sub-agents to implement features, act as a coordinator only
- After completing features (large or small), always run commands like lint, type check and next build to check code
  quality

## TESTING

- Use any testing tools, libraries available to the project for testing your changes
- Never assume your changes simply work, always test!

## UI DESIGN

- Always follow the UI design system when creating or reviewing components or pages.
- Design System: @DESIGN.md

## Codebase Exploration Tools (Prefer these over shell commands)

* bash: Execute shell commands (Git, npm, etc.) when specialized tools are insufficient.
* edit: Precisely modify the content of an existing file (string replacement).
* glob: Search for files using patterns (`*.rs`, `src/**/*.ts`).
* grep: Search file contents for a specific pattern (regex).
* read: Read the content of a file at a given offset.
* question: Ask the user questions to clarify instructions or make decisions.
* skill: Load a specialized skill if the task matches a predefined profile.
* task: Launch an autonomous agent for complex, multi-step tasks.
* todowrite: Maintain a structured to-do list for tracking complex projects.
* webfetch: Retrieve content from an external URL.
* write: Write or create a new file on the filesystem.

## RTK — Token-Optimized CLI

**rtk** prefixes shell commands to compress output (e.g., `rtk git status`).

### Meta Commands

- `rtk gain`: Token savings dashboard.
- `rtk discover`: Find missed `rtk` opportunities.

## Instructions

- **Authentication**: [authentication.instructions.md](.agents/authentication.instructions.md)
- **Backend**: [backend.instructions.md](.agents/backend.instructions.md)
- **Database Schema**: [database-schema.instructions.md](.agents/database-schema.instructions.md)
- **Frontend**: [frontend.instructions.md](.agents/frontend.instructions.md)
- **Design System**: [design-system.instructions.md](.agents/design-system.instructions.md)
- **API Endpoints**: [endpoints.instructions.md](.agents/endpoints.instructions.md)