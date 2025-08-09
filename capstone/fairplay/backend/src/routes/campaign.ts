import express from 'express';
import { finalizeCampaign } from '../services/campaignService';

export const campaignRouter = express.Router();

// POST /api/campaign/finalize
// Body: { campaignId: string }
campaignRouter.post('/finalize', async (req, res) => {
  const { campaignId } = req.body;
  try {
    const result = await finalizeCampaign(campaignId);
    res.json({ success: true, result });
  } catch (e) {
    res.status(500).json({ success: false, error: e.message });
  }
});
