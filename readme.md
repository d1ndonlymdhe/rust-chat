# Rust Chat

A full-stack chat application built entirely in Rust, featuring a custom UI framework powered by Raylib.

## Overview

Rust Chat is a real-time messaging application with a unique approach: instead of using traditional web technologies, the client uses a custom-built UI framework on top of Raylib for rendering. This provides a native, high-performance desktop experience.

## Project Structure

```
rust-chat/
├── client/    # Desktop client application
├── ui/        # Custom UI framework built on Raylib
├── server/    # Backend API server
├── shared/    # Shared types and utilities
└── macros/    # Procedural macros
```

## Client

The client is a native desktop application that provides the user interface for the chat system.

### Features

- **Authentication** - Login and signup screens
- **Dashboard** - Conversations list and search functionality
- **Routing** - Custom client-side router with navigation history
- **Session Management** - Token-based authentication with refresh support

### Architecture

The client uses a component-based architecture with:

- **Router** - A custom routing system supporting nested routes, outlets, URL parameters, and navigation history
- **State Management** - Global state stores for each feature (login, signup, conversations, search)
- **Async Operations** - Background thread handling for API calls with UI rebuild signals

### Routing System

The client implements a powerful routing system inspired by modern web frameworks like React Router.

#### Route Types

- **Container Routes** - Routes that can have nested child routes rendered via outlets
- **Leaf Routes** - Terminal routes that render final content

#### Outlets

Outlets are placeholder components where child routes get rendered. This enables nested layouts where parent routes provide shared UI (headers, sidebars) while child content changes.

```rust
// Define a container route with an outlet
Route::container(
    "auth",                           // Route name
    on_mount_callback,                // Called when route becomes active
    on_dismount_callback,             // Called when leaving route
    "auth_outlet",                    // Outlet ID for child routes
    Box::new(|params| auth_screen()), // Component builder
    vec![login_route(), signup_route()], // Child routes
)

// In the component, place the outlet where children should render
fn auth_screen() -> Component {
    Layout::get_col_builder()
        .children(vec![
            header(),
            outlet("auth_outlet"),  // Child routes render here
            footer(),
        ])
        .build()
}
```

#### Lifecycle Hooks

Routes support mount/dismount callbacks for initializing and cleaning up state:

```rust
Route::leaf(
    "login",
    Box::new(|| LoginState::init()),    // on_mount
    Box::new(|| LoginState::de_init()), // on_dismount
    Box::new(|params| login_page()),
)
```

#### URL Parameters

Routes support query parameters accessible via `RouteParams`:

```rust
// URL: dashboard/conversations?id=123&tab=messages
Router::push("dashboard/conversations?id=123&tab=messages");

// In component
fn conversations(params: RouteParams) -> Component {
    let id = params.get("id");     // Some("123")
    let tab = params.get("tab");   // Some("messages")
    // ...
}
```

#### Navigation

```rust
Router::push("dashboard/search");  // Navigate with history
Router::set("auth/login");         // Navigate without history (replace)
Router::back();                    // Go back in history
Router::can_go_back();             // Check if back is possible
```

#### Route Structure Example

```
root
├── auth/
│   ├── login      (leaf)
│   └── signup     (leaf)
└── dashboard/
    ├── conversations  (leaf)
    └── search         (leaf)
```

### Key Components

```
client/src/
├── main.rs           # App entry point and UI initialization
├── app/
│   ├── mod.rs        # Root route configuration
│   ├── auth/         # Authentication screens
│   │   ├── login.rs
│   │   └── signup.rs
│   └── dashboard/    # Main app screens
│       ├── conversations.rs
│       └── search.rs
└── utils/
    ├── router.rs     # Client-side routing
    ├── session.rs    # Auth token management
    ├── fetch.rs      # HTTP client utilities
    └── popup.rs      # Modal/popup helpers
```

## UI Framework

A custom UI framework built on top of Raylib, providing a declarative component system for building desktop interfaces.

### Core Concepts

#### Components

Components are the building blocks of the UI. Each component implements the `Base` trait which handles:
- Positioning and dimensions
- Drawing/rendering
- Mouse and keyboard event handling
- Scroll behavior

#### Layout System

The framework uses a flexbox-inspired layout system:

```rust
Layout::get_col_builder()
    .dim((Length::FILL, Length::FIT))
    .main_align(Alignment::Center)
    .gap(10)
    .children(vec![...])
    .build()
```

**Length Options:**
- `Length::FILL` - Fill available space
- `Length::FIT` - Fit to content
- `Length::FIXED(n)` - Fixed pixel size
- `Length::FillPer(n)` - Fill percentage of parent
- `Length::FitPer(n)` - Fit percentage of content

**Alignment:**
- `Alignment::Start`
- `Alignment::Center`
- `Alignment::End`

#### Available Components

| Component | Description |
|-----------|-------------|
| `Layout` | Container with row/column direction, alignment, padding, gap |
| `TextLayout` | Text display with configurable font size and color |
| `TextInput` | Editable text field with keyboard handling |
| `RawText` | Low-level text rendering |

### Event Handling

Components can respond to:
- **Mouse Events** - Click, position tracking
- **Keyboard Events** - Key press with modifier support (Shift, Ctrl)
- **Scroll Events** - Mouse wheel scrolling

```rust
Layout::get_row_builder()
    .on_click(Box::new(|mouse_event| {
        // Handle click
        true // Consume event
    }))
    .build()
```

### UI Root

The `UIRoot` manages the main render loop:
- 60 FPS target frame rate
- Automatic UI rebuild on events
- Focus management for input components
- Scroll offset tracking

```rust
UIRoot::start(
    Box::new(|| build_ui()),
    (1920, 1000),
    "Window Title",
    rebuild_signal_receiver,
);
```

### Styling

Components support various styling options:
- Background colors
- Padding (top, right, bottom, left)
- Border width and color
- Text color and font size

## Server

The backend is built with **Rocket** web framework and **PostgreSQL** database, using **SQLx** for type-safe database queries.

### Tech Stack

- **Rocket** - Fast, type-safe web framework for Rust
- **SQLx** - Async, compile-time verified SQL queries
- **PostgreSQL** - Relational database for persistent storage

### API Endpoints

#### Authentication (`/auth`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/signup` | Create a new user account |
| POST | `/auth/login` | Authenticate and receive tokens |
| POST | `/auth/refresh` | Refresh access token |

#### Users (`/users`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/users/search` | Search for users |

#### Chat (`/chat`)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/chat/conversation` | Create a new conversation |
| GET | `/chat/conversation` | Get user's conversations |

### Authentication Flow

The server implements JWT-based authentication with refresh token rotation:

1. **Login** - User credentials are verified, returns access + refresh tokens
2. **Access Token** - Short-lived token for API authentication
3. **Refresh Token** - Long-lived token to obtain new access tokens
4. **Token Family** - Tracks token lineage for security (detects token reuse)

### Database Schema

#### Users & Authentication

```
users
├── id, username, hash_password
├── created_at, updated_at

token_family
├── id, user_id
├── created_at, updated_at

token
├── id, token
├── created_at, updated_at

token_family_rel
├── token_family_id, token_id, status
```

#### Conversations & Messages

```
conversation
├── id, conv_type, title
├── created_at, updated_at

conversation_member
├── id, conversation_id, user_id, role
├── created_at, updated_at

message
├── id, conversation_id, sender_member_id
├── message_type, message_content_id
├── created_at, updated_at

text_message_content
├── id, text
├── created_at, updated_at
```

### Response Format

All API responses follow a consistent structure:

```rust
{
    "success": bool,
    "message": "...",
    "data": T  // Response payload
}
```

HTTP status codes:
- `200` - Success
- `400` - Bad Request
- `401` - Unauthorized
- `404` - Not Found
- `500` - Internal Error

### Migrations

Database migrations are managed with SQLx and run automatically on server startup:

```bash
# Migrations are in server/migrations/
20251225083202_init.up.sql        # Users & tokens
20260109164437_conversations.up.sql  # Chat tables
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL database
- Raylib dependencies

### Running the Server

```bash
cd server
cargo run
```

### Running the Client

```bash
cd client
cargo run
```

## License

MIT
