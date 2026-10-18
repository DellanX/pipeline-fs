# Pipeline-FS UI

This is the user interface for the Pipeline-FS application, a file browser for pipeline stages.

## Project Structure

```
src/
├── components/           # Reusable UI components
│   ├── StageBrowser.vue  # Component to display files in a stage
│   └── FileItemCard.vue  # Individual file card component
├── views/                # Page-level components
│   ├── HomeView.vue      # Main dashboard view
│   └── StageView.vue     # Individual stage view
├── router/               # Vue Router configuration
│   └── index.ts          # Route definitions
└── main.ts               # Application entry point
```

## Component Organization

The UI follows a card-based design approach where each distinct data item or functionality is implemented as a separate component:

- **StageBrowser.vue**: Displays files within a pipeline stage
- **FileItemCard.vue**: Represents an individual file with name, icon, and size
- **HomeView.vue**: Main dashboard showing all pipeline stages
- **StageView.vue**: Detailed view for a specific stage

## Features

- Browse files across multiple pipeline stages
- Responsive grid layout for file display
- API integration with backend services
- Vue Router navigation between views
- Light and dark themes with a persistent theme toggle

## Development

To start the development server:

```bash
npm run dev
```

To build for production:

```bash
npm run build
```