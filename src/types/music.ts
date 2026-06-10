export type AppSettings = {
  serverIp: string;
  serverPort: string;
  volume?: number;
  playbackOrder?: 'loop' | 'one' | 'shuffle';
  closeWindowBehavior?: 'minimize' | 'exit';
};

export type Track = {
  id: number;
  title: string;
  artist: string;
  coverUrl: string;
  src: string;
  duration?: string;
};
