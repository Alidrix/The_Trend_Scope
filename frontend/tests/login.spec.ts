import { test, expect } from '@playwright/test';

const baseURL = process.env.PLAYWRIGHT_TEST_BASE_URL || 'http://localhost:5173';

test('affiche le formulaire de connexion', async ({ page }) => {
  await page.goto(`${baseURL}/login`);
  await expect(page.getByText('Connectez-vous à votre radar de tendances')).toBeVisible();
  await expect(page.getByPlaceholder('votre@email.com')).toBeVisible();
  await expect(page.getByPlaceholder('Votre mot de passe')).toBeVisible();
});
