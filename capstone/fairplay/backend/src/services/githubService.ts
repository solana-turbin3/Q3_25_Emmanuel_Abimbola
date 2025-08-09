import { Octokit } from '@octokit/rest';

// Scoring model from README
const SCORE_MERGED = 100;
const SCORE_UNMERGED = 20;
const SCORE_SPAM = 0;

// TODO: Add spam detection logic if needed

export async function fetchPRsAndScore(repo: string, campaignId: string) {
  // Security: Check for valid GitHub token
  if (!process.env.GITHUB_TOKEN) {
    throw new Error('Missing GITHUB_TOKEN in environment');
  }

  // Security: Validate repo format (owner/repo)
  if (!/^\w[\w-]*\/\w[\w-]*$/.test(repo)) {
    throw new Error('Invalid repo format. Expected owner/repo');
  }

  const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });
  const [owner, repoName] = repo.split('/');
  const pulls = await octokit.pulls.list({ owner, repo: repoName, state: 'all', per_page: 100 });

  // Map: username -> score
  const scores: Record<string, number> = {};

  for (const pr of pulls.data) {
    const username = pr.user?.login;
    if (!username) continue;

    // Basic spam/invalid detection (expand as needed)
    const isSpam =
      pr.title.toLowerCase().includes('spam') ||
      pr.body?.toLowerCase().includes('spam') ||
      pr.additions + pr.deletions < 2; // trivial PR

    let score = 0;
    if (isSpam) {
      score = SCORE_SPAM;
    } else if (pr.merged_at) {
      score = SCORE_MERGED;
    } else if (pr.state === 'closed') {
      score = SCORE_UNMERGED;
    } else {
      score = SCORE_UNMERGED; // Optionally refine
    }
    scores[username] = (scores[username] || 0) + score;
  }

  // Integration: Here, scores can be sent to the Solana program using update_score instruction
  // (see solanaService.ts for placeholder)

  return scores;
}
