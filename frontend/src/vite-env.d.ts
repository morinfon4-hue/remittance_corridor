/// <reference types="vite/client" />

interface Window {
  freighterApi?: {
    isConnected?: () => Promise<boolean>;
    getPublicKey?: () => Promise<string>;
    signTransaction?: (xdr: string, opts?: { networkPassphrase?: string }) => Promise<string>;
  };
}
