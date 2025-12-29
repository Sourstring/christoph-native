## Architecture Plan

### Phase 1: Core SFTP (Rust)
- [ ] Set up ssh2 crate
- [ ] Create connection function
- [ ] Test connecting to a real SFTP server
- [ ] Implement list directory
- [ ] Implement download file

### Phase 2: Tauri Commands (Bridge)
- [ ] Wrap SFTP functions as Tauri commands
- [ ] Handle errors properly
- [ ] Test from Tauri's dev tools

### Phase 3: Basic Frontend (React)
- [ ] Connection form (host, username, password)
- [ ] "Connect" button that calls your Tauri command
- [ ] Display list of files
- [ ] Show errors if connection fails

### Phase 4: Polish
- [ ] File tree UI
- [ ] Upload/download buttons
- [ ] Progress indicators
- [ ] Save connections