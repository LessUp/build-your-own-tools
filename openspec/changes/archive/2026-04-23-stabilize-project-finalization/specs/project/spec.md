## MODIFIED Requirements

### Requirement: OpenSpec Governance

The project SHALL use OpenSpec as the canonical planning layer for repository-wide changes, and the active change backlog SHALL reflect only currently intended work.

#### Scenario: Starting repository-wide work
- **GIVEN** a repo-wide feature, cleanup, or stabilization effort
- **WHEN** work begins
- **THEN** an OpenSpec change SHALL define the proposal, specs, design, and tasks before implementation proceeds

#### Scenario: Removing drift from active changes
- **GIVEN** an active OpenSpec change that no longer matches project goals
- **WHEN** the maintainers are prioritizing close-out work
- **THEN** the stale or out-of-scope change SHALL be removed, archived, or otherwise excluded from the active delivery backlog

### Requirement: Documentation Architecture

The project SHALL maintain a small, purposeful documentation set in which each major document has a distinct role and stale or duplicated content is aggressively removed.

#### Scenario: Root README
- **GIVEN** a new visitor opening the repository
- **WHEN** they read `README.md`
- **THEN** they SHALL quickly understand the project value, current tool set, learning angle, and how to navigate deeper docs

#### Scenario: Supporting docs
- **GIVEN** architecture, contribution, process, or changelog documentation
- **WHEN** maintainers revise the docs
- **THEN** they SHALL keep one canonical explanation per topic instead of repeating whole sections across multiple files

### Requirement: AI-Assisted Development Workflow

The project SHALL provide explicit, project-level guidance for AGENTS, Claude, Copilot, and OpenCode usage, with OpenSpec-first execution and lightweight review discipline.

#### Scenario: Starting a code change with AI assistance
- **GIVEN** a maintainer is using AI tooling on the repository
- **WHEN** work begins
- **THEN** the project instructions SHALL direct the maintainer to start from OpenSpec artifacts and follow the documented repo workflow

#### Scenario: Review and execution strategy
- **GIVEN** a maintainer is choosing among direct edits, subagents, `/review`, autopilot, or `/fleet`
- **WHEN** they follow the project guidance
- **THEN** the documented workflow SHALL prefer review before merge, use autopilot for long sequential work, and avoid `/fleet` unless there is a clear payoff

### Requirement: Automation Governance

The project SHALL keep only meaningful automation, with tracked canonical configuration and narrow workflow triggers that avoid noisy or low-value runs.

#### Scenario: Workflow design
- **GIVEN** a GitHub Actions workflow in this repository
- **WHEN** it is reviewed or updated
- **THEN** its trigger scope, job purpose, and configuration ownership SHALL be clear and proportionate to the repository's maintenance goals

#### Scenario: Security or license tooling
- **GIVEN** security or dependency policy checks are needed
- **WHEN** workflows execute them
- **THEN** canonical configuration files SHOULD live in the repository instead of being generated ad hoc inside workflow steps when that improves maintainability

### Requirement: Public Project Presentation

The project SHALL keep GitHub Pages and GitHub repository metadata aligned so that the public presentation is concise, attractive, and representative of the finished project.

#### Scenario: GitHub Pages landing page
- **GIVEN** a visitor lands on the Pages site
- **WHEN** they view the homepage
- **THEN** they SHALL see a focused explanation of the project's tools, learning path, and differentiators instead of a weak mirror of the README

#### Scenario: GitHub repository About section
- **GIVEN** the repository About section on GitHub
- **WHEN** metadata is finalized
- **THEN** its description, topics, and homepage URL SHALL align with the site and the intended positioning of the project

### Requirement: Finalization Readiness

The project SHALL optimize the remaining work for high-quality completion and low-maintenance operation rather than for continued feature expansion.

#### Scenario: Evaluating new work
- **GIVEN** a proposed repo-wide change during the stabilization phase
- **WHEN** maintainers decide whether to include it
- **THEN** they SHALL prefer bug fixes, cleanup, governance, and presentation work over new tool or feature expansion
