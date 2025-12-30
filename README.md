## Architecture Plan

### Phase 1: Core SFTP (Rust)
- [ x ] Set up ssh2 crate
- [ x ] Create connection function
- [ x ] Test connecting to a real SFTP server
- [ x ] Implement list directory
- [ x ] Implement download file

### Phase 2: Tauri Commands (Bridge)
- [ x ] Wrap SFTP functions as Tauri commands
- [ x ] Handle errors properly
- [ x ] Test from Tauri's dev tools

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