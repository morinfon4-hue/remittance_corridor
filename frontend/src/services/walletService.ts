export async function connectFreighter(): Promise<string> {
  if (!window.freighterApi) {
    throw new Error("Freighter wallet is not available in this browser.");
  }

  const isConnected = window.freighterApi.isConnected
    ? await window.freighterApi.isConnected()
    : false;

  if (!isConnected) {
    throw new Error("Please connect Freighter, then try again.");
  }

  if (!window.freighterApi.getPublicKey) {
    throw new Error("Freighter public key method is not available.");
  }

  return window.freighterApi.getPublicKey();
}

export function shortenAddress(address: string): string {
  if (address.length <= 12) {
    return address;
  }

  return address.slice(0, 6) + "..." + address.slice(-6);
}
