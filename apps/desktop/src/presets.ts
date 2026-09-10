/**
 * Built-in species presets, bundled straight from the repository's
 * `presets/species/` TOML documents — the same files the CLI consumes.
 */

import oakToml from '../../../presets/species/oak.toml?raw';
import palmToml from '../../../presets/species/palm.toml?raw';
import pineToml from '../../../presets/species/pine.toml?raw';
import willowToml from '../../../presets/species/willow.toml?raw';

export interface SpeciesPreset {
  id: string;
  label: string;
  toml: string;
}

export const PRESETS: SpeciesPreset[] = [
  { id: 'oak', label: 'Oak', toml: oakToml },
  { id: 'pine', label: 'Pine', toml: pineToml },
  { id: 'palm', label: 'Palm', toml: palmToml },
  { id: 'willow', label: 'Willow', toml: willowToml },
];
