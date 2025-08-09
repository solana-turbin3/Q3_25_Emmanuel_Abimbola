import express from 'express';
import { fetchPRsAndScore } from '../services/githubService';

export const githubRouter = express.Router();

// POST /api/github/sync
// Body: { repo: string, campaignId: string }
githubRouter.post('/sync', async (req, res) => {
  const { repo, campaignId } = req.body;
  try {
    const scores = await fetchPRsAndScore(repo, campaignId);
    res.json({ success: true, scores });
  } catch (e) {
    res.status(500).json({ success: false, error: e.message });
  }
});
