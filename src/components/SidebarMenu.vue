<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <p class="eyebrow">本地音乐</p>
      <h2>Library</h2>
    </div>

    <div class="search-wrapper">
      <input type="search" placeholder="Search local songs..." class="search-input" />
    </div>

    <nav class="menu-list">
      <button class="menu-item active">歌单</button>
      <button class="menu-item">设置</button>
    </nav>

    <div class="playlist-group">
      <p class="group-label">PLAYLISTS</p>
      <button class="playlist-item active">Good</button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 260px;
  padding: 1.5rem;
  border-radius: 0;
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(15, 23, 42, 0.08);
  box-shadow: 0 24px 80px rgba(15, 23, 42, 0.08);
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.sidebar-header h2 {
  margin: 0.35rem 0 0;
  font-size: 1.6rem;
}
.eyebrow {
  margin: 0;
  font-size: 0.8rem;
  letter-spacing: 0.2em;
  color: #6366f1;
}
.search-wrapper {
  display: flex;
}
.search-input {
  width: 100%;
  border: 1px solid rgba(15, 23, 42, 0.12);
  border-radius: 0;
  padding: 0.9rem 1rem;
  background: rgba(243, 244, 246, 0.9);
  color: #111827;
}
.menu-list,
.playlist-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}
.menu-item,
.playlist-item {
  all: unset;
  cursor: pointer;
  padding: 0.95rem 1rem;
  border-radius: 0;
  background: rgba(247, 250, 252, 0.9);
  color: #111827;
  font-weight: 600;
  transition: background 0.2s ease;
}
.menu-item:hover,
.playlist-item:hover {
  background: rgba(99, 102, 241, 0.08);
}
.menu-item.active,
.playlist-item.active {
  background: rgba(79, 70, 229, 0.1);
  color: #3730a3;
}
.group-label {
  margin: 0;
  font-size: 0.75rem;
  color: #6b7280;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}
</style>
