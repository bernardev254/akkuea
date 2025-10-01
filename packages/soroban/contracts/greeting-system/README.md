# Greeting System Smart Contract

A Soroban smart contract for the Stellar network that enables users to create greetings and engage with them through social interactions (likes and comments).

## Overview

This contract implements a complete greeting system with social engagement features:
- **Greeting Creation**: Users can create greetings with custom messages
- **Like System**: Users can like greetings (with duplicate prevention)
- **Comment System**: Users can comment on greetings with validation and moderation
- **Authentication**: All interactions require Stellar account authentication
- **Event Emissions**: All actions emit events for off-chain tracking

## Features

### Core Functionality

1. **Greeting Management**
   - Create greetings with unique IDs
   - Retrieve greetings by ID
   - Check greeting existence
   - Get total greeting count

2. **Social Interactions**
   - Like greetings (one like per user per greeting)
   - Comment on greetings (multiple comments allowed)
   - View like counts
   - View all comments on a greeting
   - Check if a user has liked a greeting

3. **Security & Validation**
   - Authentication required for all write operations
   - Message validation (1-1000 characters)
   - Comment validation (1-500 characters)
   - Duplicate like prevention
   - Comment limit (100 per greeting)
   - Basic content moderation

## Contract Structure

```
greeting-system/
├── src/
│   ├── lib.rs          # Main contract entry point
│   ├── social.rs       # Social interaction logic
│   ├── utils.rs        # Validation and utility functions
│   └── test.rs         # Comprehensive test suite
├── Cargo.toml          # Package configuration
└── README.md           # This file
```

## Data Structures

### Greeting
```rust
pub struct Greeting {
    pub id: u64,           // Unique greeting ID
    pub creator: Address,  // Creator's Stellar address
    pub message: String,   // Greeting message
    pub timestamp: u64,    // Creation timestamp
}
```

### SocialInteraction
```rust
pub struct SocialInteraction {
    pub greeting_id: u64,      // Associated greeting ID
    pub user: Address,         // User's Stellar address
    pub action: String,        // "like" or "comment"
    pub comment_text: String,  // Comment content (empty for likes)
    pub timestamp: u64,        // Interaction timestamp
}
```

## Contract Functions

### Initialization

#### `initialize(env: Env)`
Initializes the contract and sets up the greeting counter.

**Parameters:**
- `env`: The contract environment

**Returns:** None

---

### Greeting Functions

#### `create_greeting(env: Env, creator: Address, message: String) -> u64`
Creates a new greeting.

**Parameters:**
- `env`: The contract environment
- `creator`: The Stellar address of the greeting creator
- `message`: The greeting message (1-1000 characters)

**Returns:** The unique greeting ID

**Authentication:** Required (creator must authenticate)

**Events:** Emits `grt_crtd` event with greeting details

**Panics:**
- If message is empty
- If message exceeds 1000 characters

---

#### `get_greeting(env: Env, greeting_id: u64) -> Greeting`
Retrieves a greeting by its ID.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The unique greeting ID

**Returns:** The greeting data

**Panics:** If greeting doesn't exist

---

#### `greeting_exists(env: Env, greeting_id: u64) -> bool`
Checks if a greeting exists.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting ID to check

**Returns:** `true` if greeting exists, `false` otherwise

---

#### `get_greeting_count(env: Env) -> u64`
Gets the total number of greetings created.

**Parameters:**
- `env`: The contract environment

**Returns:** Total greeting count

---

### Social Interaction Functions

#### `like_greeting(env: Env, greeting_id: u64, user: Address) -> u64`
Likes a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting to like
- `user`: The user liking the greeting

**Returns:** Updated like count

**Authentication:** Required (user must authenticate)

**Events:** Emits `like` event

**Panics:**
- If greeting doesn't exist
- If user has already liked the greeting

---

#### `comment_on_greeting(env: Env, greeting_id: u64, user: Address, text: String) -> u64`
Adds a comment to a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting to comment on
- `user`: The user commenting
- `text`: The comment text (1-500 characters)

**Returns:** New comment count

**Authentication:** Required (user must authenticate)

**Events:** Emits `comment` event

**Panics:**
- If greeting doesn't exist
- If comment is empty
- If comment exceeds 500 characters
- If comment limit (100) is reached
- If comment contains inappropriate content

---

#### `get_like_count(env: Env, greeting_id: u64) -> u64`
Gets the like count for a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting ID

**Returns:** Like count (0 if no likes)

---

#### `get_comments(env: Env, greeting_id: u64) -> Vec<SocialInteraction>`
Gets all comments for a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting ID

**Returns:** Vector of comments (empty if no comments)

---

#### `get_comment_count(env: Env, greeting_id: u64) -> u64`
Gets the comment count for a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting ID

**Returns:** Comment count

---

#### `has_user_liked(env: Env, greeting_id: u64, user: Address) -> bool`
Checks if a user has liked a greeting.

**Parameters:**
- `env`: The contract environment
- `greeting_id`: The greeting ID
- `user`: The user address

**Returns:** `true` if user has liked, `false` otherwise

---

## Storage Keys

The contract uses the following storage keys:

- `GRT_CNT`: Greeting counter
- `GRT`: Greeting data (with greeting ID)
- `LIKE_CNT`: Like count per greeting
- `LIKE_TRK`: Like tracker (greeting ID + user address)
- `COMMENTS`: Comments per greeting

## Events

### Greeting Created Event
```
Topic: "grt_crtd"
Data: (greeting_id, creator, message, timestamp)
```

### Like Event
```
Topic: "like"
Data: (greeting_id, user, timestamp)
```

### Comment Event
```
Topic: "comment"
Data: (greeting_id, user, comment_text, timestamp)
```

## Testing

The contract includes comprehensive tests covering:

-  Greeting creation and retrieval
-  Like functionality with duplicate prevention
-  Comment validation and storage
-  View functions
-  Authentication checks
-  Edge cases and error handling
-  Integration workflows

Run tests with:
```bash
cd packages/soroban/contracts/greeting-system
cargo test
```

**Test Results:** All 21 tests passing 

## Gas Optimization

The contract is optimized for gas efficiency:

- Uses `symbol_short!` for storage keys (7 characters max)
- Efficient storage with tuple keys
- Minimal storage reads/writes
- Optimized data structures

## Security Considerations

1. **Authentication**: All write operations require `require_auth()`
2. **Input Validation**: All inputs are validated for length and content
3. **Duplicate Prevention**: Like tracking prevents duplicate likes
4. **Rate Limiting**: Placeholder for future rate limiting implementation
5. **Content Moderation**: Basic profanity filtering (extensible)

## Future Enhancements

The contract includes placeholders for:

- Advanced content moderation (external oracle integration)
- Rate limiting per user
- Comment pagination for large comment threads
- Reputation-based features
- User whitelisting/blacklisting

## Building

Build the contract:
```bash
cd packages/soroban/contracts/greeting-system
cargo build --target wasm32-unknown-unknown --release
```

## Deployment

Deploy to Stellar testnet:
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/greeting_system.wasm \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

## License

This contract is part of the Akkuea project.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting PRs.

