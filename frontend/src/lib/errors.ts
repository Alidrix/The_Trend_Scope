export const getErrorMessage = (error: unknown, fallback = 'Erreur inconnue') =>
  error instanceof Error ? error.message : fallback;
