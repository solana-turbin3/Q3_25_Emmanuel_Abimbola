import { fetchPRsAndScore } from './githubService';
// import { updateScoreOnChain } from './solanaService'; // To be implemented

export async function finalizeCampaign(campaignId: string) {
  // Fetch campaign metadata (e.g., repo) from DB or on-chain (mocked here)
  const repo = 'owner/repo'; // TODO: Replace with actual lookup
  const scores = await fetchPRsAndScore(repo, campaignId);
  // await updateScoreOnChain(campaignId, scores); // To be implemented
  return { campaignId, scores };
}
