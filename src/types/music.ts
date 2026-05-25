export type AppSettings = {
  serverIp: string;
  serverPort: string;
};

export type Track = {
  id: number;
  title: string;
  artist: string;
  coverUrl: string;
  src: string;
  duration?: string;
};
