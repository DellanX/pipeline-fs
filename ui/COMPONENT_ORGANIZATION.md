# Component Organization Policy

This document outlines the component organization and structure guidelines for the Pipeline-FS UI application.

## Component Structure

All Vue components should be organized under `/src/components/` directory. Each component should be self-contained with:
- A single `.vue` file that includes template, script, and style sections
- Clear, descriptive naming
- Reusable design patterns

## Card-Based Design Approach

### Rule: "Cards as Components"
All UI elements that represent distinct data items or functionality should be implemented as individual components.

### Examples of Card Components:
1. **FileItemCard.vue** - Displays a single file with name, icon, and size
2. **StageCard.vue** - Represents a pipeline stage with its configuration
3. **PipelineInfoCard.vue** - Shows overall pipeline information
4. **ControlCard.vue** - Contains control buttons and options

### Benefits:
- Improved reusability across different views
- Easier testing and maintenance
- Better separation of concerns
- Enhanced developer experience when building new features

## Component Naming Convention

Use PascalCase for component names:
- `StageBrowser.vue`
- `FileItemCard.vue`
- `PipelineInfoCard.vue`

## Component Composition

1. **Presentational Components**: Pure UI components that only handle display logic
2. **Container Components**: Components that manage data and pass it to presentational components
3. **Smart Components**: Combine both presentation and data management when appropriate

## File Structure Example

```
src/
├── components/
│   ├── StageBrowser.vue
│   ├── FileItemCard.vue
│   ├── PipelineInfoCard.vue
│   └── ControlCard.vue
├── views/
│   ├── HomeView.vue
│   └── StageView.vue
└── router/
    └── index.ts
```

## Implementation Guidelines

1. Each component should be focused on a single responsibility
2. Use props for passing data into components
3. Use events for communication from child to parent components
4. Prefer composition over inheritance where possible
5. Keep components small and focused
6. Reuse existing components rather than duplicating functionality

## Styling Approach

1. Use Tailwind CSS classes for styling
2. Avoid inline styles in components
3. Use consistent naming conventions for classes
4. Leverage component-level scoping to prevent CSS conflicts