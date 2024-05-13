import {test, expect} from '@playwright/test'

test('has title', async ({page}) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Dual Lists' }).click()
  await expect(page.getByRole('heading', { name: 'Dual List' })).toBeVisible();
})
