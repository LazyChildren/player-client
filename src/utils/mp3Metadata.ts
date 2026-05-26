export type Mp3Metadata = {
  title: string;
  artist: string;
};

const ID3_HEADER_SIZE = 10;
const ID3V1_TAG_SIZE = 128;

export async function parseMp3Metadata(file: File): Promise<Mp3Metadata> {
  const id3v2 = await parseId3v2(file);

  if (id3v2.title || id3v2.artist) {
    return id3v2;
  }

  return parseId3v1(file);
}

async function parseId3v2(file: File): Promise<Mp3Metadata> {
  const headerBuffer = await file.slice(0, ID3_HEADER_SIZE).arrayBuffer();
  const header = new Uint8Array(headerBuffer);

  if (!isId3v2Header(header)) {
    return { title: "", artist: "" };
  }

  const version = header[3];
  const tagSize = decodeSynchsafeInteger(header.slice(6, 10));
  const tagBytes = new Uint8Array(await file.slice(ID3_HEADER_SIZE, ID3_HEADER_SIZE + tagSize).arrayBuffer());

  let offset = 0;
  let title = "";
  let artist = "";

  while (offset + 10 <= tagBytes.length) {
    const frameHeader = tagBytes.slice(offset, offset + 10);
    const frameId = decodeLatin1(frameHeader.slice(0, 4)).replace(/\0/g, "");

    if (!frameId.trim()) {
      break;
    }

    const frameSize =
      version === 4
        ? decodeSynchsafeInteger(frameHeader.slice(4, 8))
        : decodeBigEndianInteger(frameHeader.slice(4, 8));

    if (frameSize <= 0 || offset + 10 + frameSize > tagBytes.length) {
      break;
    }

    const frameData = tagBytes.slice(offset + 10, offset + 10 + frameSize);

    if (frameId === "TIT2") {
      title = decodeTextFrame(frameData);
    } else if (frameId === "TPE1") {
      artist = decodeTextFrame(frameData);
    }

    if (title && artist) {
      break;
    }

    offset += 10 + frameSize;
  }

  return { title, artist };
}

async function parseId3v1(file: File): Promise<Mp3Metadata> {
  if (file.size < ID3V1_TAG_SIZE) {
    return { title: "", artist: "" };
  }

  const buffer = await file.slice(file.size - ID3V1_TAG_SIZE).arrayBuffer();
  const bytes = new Uint8Array(buffer);

  if (decodeLatin1(bytes.slice(0, 3)) !== "TAG") {
    return { title: "", artist: "" };
  }

  return {
    title: decodeLatin1(bytes.slice(3, 33)).replace(/\0/g, "").trim(),
    artist: decodeLatin1(bytes.slice(33, 63)).replace(/\0/g, "").trim(),
  };
}

function isId3v2Header(bytes: Uint8Array) {
  return bytes.length >= ID3_HEADER_SIZE && bytes[0] === 0x49 && bytes[1] === 0x44 && bytes[2] === 0x33;
}

function decodeSynchsafeInteger(bytes: Uint8Array) {
  return ((bytes[0] & 0x7f) << 21) | ((bytes[1] & 0x7f) << 14) | ((bytes[2] & 0x7f) << 7) | (bytes[3] & 0x7f);
}

function decodeBigEndianInteger(bytes: Uint8Array) {
  return (bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3];
}

function decodeTextFrame(frameData: Uint8Array) {
  if (!frameData.length) {
    return "";
  }

  const encoding = frameData[0];
  const content = frameData.slice(1);

  switch (encoding) {
    case 0:
      return decodeLatin1(content).replace(/\0/g, "").trim();
    case 1:
      return decodeUtf16WithBom(content).replace(/\0/g, "").trim();
    case 2:
      return decodeUtf16Be(content).replace(/\0/g, "").trim();
    case 3:
      return new TextDecoder("utf-8").decode(content).replace(/\0/g, "").trim();
    default:
      return "";
  }
}

function decodeLatin1(bytes: Uint8Array) {
  return new TextDecoder("iso-8859-1").decode(bytes);
}

function decodeUtf16WithBom(bytes: Uint8Array) {
  if (bytes.length < 2) {
    return "";
  }

  if (bytes[0] === 0xfe && bytes[1] === 0xff) {
    return decodeUtf16Be(bytes.slice(2));
  }

  if (bytes[0] === 0xff && bytes[1] === 0xfe) {
    return new TextDecoder("utf-16le").decode(bytes.slice(2));
  }

  return new TextDecoder("utf-16le").decode(bytes);
}

function decodeUtf16Be(bytes: Uint8Array) {
  if (bytes.length < 2) {
    return "";
  }

  const normalized = new Uint8Array(bytes.length);

  for (let index = 0; index < bytes.length - 1; index += 2) {
    normalized[index] = bytes[index + 1];
    normalized[index + 1] = bytes[index];
  }

  return new TextDecoder("utf-16le").decode(normalized);
}
