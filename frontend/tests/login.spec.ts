import { test, expect } from '@playwright/test';

const baseURL = process.env.PLAYWRIGHT_TEST_BASE_URL || 'http://localhost:5173';

test('affiche le formulaire de connexion', async ({ page }) => {
  await page.goto(`${baseURL}/login`);
  await expect(page.getByText('Connexion')).toBeVisible();
  await expect(page.getByPlaceholder('admin')).toBeVisible();
  await expect(page.getByPlaceholder('••••••••')).toBeVisible();
});
