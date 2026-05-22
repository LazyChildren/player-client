<template>
  <aside class="sidebar">
    <div class="playlist-group">
      <p class="eyebrow">收藏夹</p>
      <button class="playlist-item active">默认收藏夹</button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 260px;
  padding: 1.5rem;
  border-radius: 0;
  background: rgba(128, 128, 128, 0.01);
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-height: 100%;
  height: 100%;
  box-sizing: border-box;
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
.playlist-group {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.menu-item,
.playlist-item {
  all: unset;
  cursor: pointer;
  padding: 0.95rem 1rem;
  border-radius: 0;
  background: rgba(247, 250, 252, 0.9);
  color: #111827;
  font-weight: 400;
  transition: background 0.2s ease;
  font-size: 0.75rem;
}

.playlist-item {
  border-radius: 0.75rem;
}
.menu-item:hover,
.playlist-item:hover {
  background: rgba(99, 102, 241, 0.08);
}
.menu-item.active,
.playlist-item.active {
  background: rgba(79, 70, 229, 0.1);
  color: #726ae6;
}
.group-label {
  margin: 0;
  font-size: 0.75rem;
  color: #6b7280;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}
</style>
