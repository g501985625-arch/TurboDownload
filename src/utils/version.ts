/**
 * Semantic Versioning (SemVer) comparison utilities
 */

export interface SemVer {
  major: number;
  minor: number;
  patch: number;
  prerelease?: string;
  build?: string;
}

/**
 * Parse a version string into its components
 */
export function parseSemVer(version: string): SemVer | null {
  const regex = /^(\d+)\.(\d+)\.(\d+)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?$/;
  const match = version.match(regex);
  
  if (!match) {
    return null;
  }
  
  return {
    major: parseInt(match[1], 10),
    minor: parseInt(match[2], 10),
    patch: parseInt(match[3], 10),
    prerelease: match[4],
    build: match[5]
  };
}

/**
 * Compare two semantic versions
 * @returns 1 if a > b, -1 if a < b, 0 if equal
 */
export function compareSemVer(a: string, b: string): number {
  const verA = parseSemVer(a);
  const verB = parseSemVer(b);
  
  if (!verA || !verB) {
    throw new Error(`Invalid version string: ${a} or ${b}`);
  }
  
  // Compare major version
  if (verA.major !== verB.major) {
    return verA.major > verB.major ? 1 : -1;
  }
  
  // Compare minor version
  if (verA.minor !== verB.minor) {
    return verA.minor > verB.minor ? 1 : -1;
  }
  
  // Compare patch version
  if (verA.patch !== verB.patch) {
    return verA.patch > verB.patch ? 1 : -1;
  }
  
  // If we have prerelease versions, they are considered less than release versions
  if (verA.prerelease && !verB.prerelease) {
    return -1;
  }
  if (!verA.prerelease && verB.prerelease) {
    return 1;
  }
  if (verA.prerelease && verB.prerelease) {
    // Compare prerelease identifiers
    const partsA = verA.prerelease.split('.');
    const partsB = verB.prerelease.split('.');
    
    const minLength = Math.min(partsA.length, partsB.length);
    for (let i = 0; i < minLength; i++) {
      const partA = partsA[i];
      const partB = partsB[i];
      
      // Check if both parts are numeric
      const numA = parseInt(partA, 10);
      const numB = parseInt(partB, 10);
      const isNumA = !isNaN(numA) && /^\d+$/.test(partA);
      const isNumB = !isNaN(numB) && /^\d+$/.test(partB);
      
      if (isNumA && isNumB) {
        if (numA !== numB) {
          return numA > numB ? 1 : -1;
        }
      } else if (isNumA && !isNumB) {
        // Numeric identifiers always have lower precedence than non-numeric ones
        return -1;
      } else if (!isNumA && isNumB) {
        // Non-numeric identifiers have higher precedence than numeric ones
        return 1;
      } else {
        // Both are non-numeric, compare lexicographically
        if (partA !== partB) {
          return partA > partB ? 1 : -1;
        }
      }
    }
    
    // If one has more parts, it's greater
    return partsA.length > partsB.length ? 1 : partsA.length < partsB.length ? -1 : 0;
  }
  
  // Versions are equal
  return 0;
}

/**
 * Check if a version is newer than another
 */
export function isNewerVersion(version: string, compareTo: string): boolean {
  return compareSemVer(version, compareTo) > 0;
}

/**
 * Check if a version should be skipped based on the skip list
 */
export function shouldSkipVersion(version: string, skippedVersions: string[]): boolean {
  return skippedVersions.includes(version);
}