# Project Standards Specification

## Purpose

Build Your Own Tools is a learning-focused repository that re-implements a small set of CLI tools in Rust and Go. This specification defines how the project is structured, maintained, and finalized as it approaches a low-maintenance, archive-ready state.

## Requirements

### Requirement: Repository Scope

The project SHALL focus on the existing tool set and SHALL treat stabilization of the current codebase as a higher priority than expanding to new tools.

#### Scenario: Existing tool matrix
- **GIVEN** the current repository layout
- **WHEN** maintainers evaluate supported tools
- **THEN** the project SHALL center on `dos2unix`, `gzip`, and `htop`
- **AND** language/platform differences SHALL be documented clearly where they exist

#### Scenario: Evaluating new scope
- **GIVEN** a proposed new tool or major feature
- **WHEN** the repository is in close-out mode
- **THEN** bug fixes, cleanup, governance, and presentation work SHALL take precedence over expansion

### Requirement: OpenSpec Lifecycle

The project SHALL use OpenSpec as the canonical planning workflow for repository-wide changes and SHALL keep the active change backlog aligned with actual priorities.

#### Scenario: Starting non-trivial work
- **GIVEN** a feature, cleanup, or process change that affects repo behavior
- **WHEN** work begins
- **THEN** an OpenSpec change SHALL capture proposal, spec, design, and tasks before implementation proceeds

#### Scenario: Starting a close-out phase with no active change
- **GIVEN** the repository is in close-out mode
- **AND** no active cleanup change exists
- **WHEN** maintainers begin the next close-out phase
- **THEN** they SHALL create a new phase-scoped change before implementation
- **AND** they SHALL NOT treat an archived change as the active source of scope

#### Scenario: Removing stale active changes
- **GIVEN** an OpenSpec change that no longer matches project priorities
- **WHEN** maintainers review the active backlog
- **THEN** the stale or out-of-scope change SHALL be removed from active delivery scope

### Requirement: Documentation Architecture

The project SHALL maintain a small, durable documentation set with one canonical explanation per topic.

#### Scenario: Root README
- **GIVEN** a new visitor opening the repository
- **WHEN** they read `README.md`
- **THEN** they SHALL quickly understand the project, the available tools, the learning angle, and where to go next

#### Scenario: Supporting documentation
- **GIVEN** architecture, contribution, workflow, or changelog documentation
- **WHEN** maintainers revise the docs
- **THEN** duplicated or stale explanations SHALL be pruned
- **AND** each document SHALL have a distinct purpose

#### Scenario: Changelog helper surfaces
- **GIVEN** changelog index, migration, or helper pages
- **WHEN** maintainers evaluate their ongoing value
- **THEN** only the surfaces that still justify maintenance SHALL remain public
- **AND** low-value helper layers SHALL be removed or collapsed into a canonical project history surface

### Requirement: AI-Assisted Workflow

The project SHALL provide explicit project-level guidance for AGENTS, Claude, Copilot, and OpenCode usage.

#### Scenario: Starting AI-assisted development
- **GIVEN** a maintainer is using AI tooling in the repository
- **WHEN** they begin work
- **THEN** the shared instruction files SHALL direct them to start from OpenSpec artifacts and follow the documented repo workflow

#### Scenario: Review strategy
- **GIVEN** a maintainer is choosing how to validate cross-cutting work
- **WHEN** they follow project guidance
- **THEN** `/review` SHALL be the preferred review step before merge-ready workflow or governance changes
- **AND** `/fleet` SHOULD be used sparingly

### Requirement: Quality Gates

The project SHALL preserve meaningful shared verification commands for code and documentation.

#### Scenario: Code verification
- **GIVEN** Rust or Go implementation changes
- **WHEN** maintainers validate them
- **THEN** `make lint-all` and `make test-all` SHALL remain valid repository-wide entry points

#### Scenario: Docs and site verification
- **GIVEN** docs, Pages, or configuration changes
- **WHEN** maintainers validate them
- **THEN** `npm run docs:check` and `npm run docs:build` SHALL be available and working

### Requirement: Automation Governance

The project SHALL keep only meaningful automation and SHALL avoid noisy or ambiguous workflow behavior.

#### Scenario: Workflow design
- **GIVEN** a workflow in `.github/workflows/`
- **WHEN** it is added or updated
- **THEN** its trigger scope, quality goal, and ownership SHALL be clear
- **AND** low-value or overly broad triggers SHALL be avoided

#### Scenario: Tracked configuration
- **GIVEN** security, dependency, or policy tooling needs configuration
- **WHEN** that tooling is automated
- **THEN** canonical config SHOULD live in tracked repository files when that improves clarity and maintainability

### Requirement: Public Project Presentation

The project SHALL keep GitHub Pages and GitHub repository metadata aligned with the intended public narrative.

#### Scenario: Pages landing page
- **GIVEN** a visitor lands on the Pages homepage
- **WHEN** they view the page
- **THEN** they SHALL see a focused explanation of the tool set, learning path, and project differentiators
- **AND** the page SHALL not function as a weak mirror of the README

#### Scenario: GitHub repository About section
- **GIVEN** the repository About section
- **WHEN** metadata is updated
- **THEN** description, topics, and homepage URL SHALL align with the finalized site and project positioning

### Requirement: Engineering Configuration

The project SHALL keep shared hooks, editor recommendations, and configuration files minimal, explicit, and project-specific.

#### Scenario: Shared configuration
- **GIVEN** a formatter, hook, LSP setting, or repo automation rule is committed
- **WHEN** maintainers review it
- **THEN** it SHALL provide clear value for this repository
- **AND** generic or redundant configuration SHALL be removed
