import express from 'express';
import dotenv from 'dotenv';
import { githubRouter } from './routes/github';
import { campaignRouter } from './routes/campaign';

dotenv.config();

const app = express();
app.use(express.json());

app.use('/api/github', githubRouter);
app.use('/api/campaign', campaignRouter);

const PORT = process.env.PORT || 4000;
app.listen(PORT, () => {
  console.log(`FairPlay backend running on port ${PORT}`);
});
