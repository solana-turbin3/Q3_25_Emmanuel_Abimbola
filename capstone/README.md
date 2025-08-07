
# 🧠 FairPlay Protocol – Proof of Concept (PoC)

FairPlay Protocol is an on-chain reward distribution system that ensures fair and transparent contribution-based rewards in collaborative environments. This PoC targets hackathons and DevRel-led bounty campaigns, with GitHub Pull Requests as the contribution metric.

## 📌 Objective

Enable campaign organizers (DevRels, Sponsors) to:
- Launch contribution-based campaigns with a set deadline and reward pool (in USDC).
- Automatically track contributors' GitHub Pull Requests.
- Score contributors based on merged/unmerged status.
- Distribute on-chain rewards fairly using a transparent scoring and allocation system.

## 🚀 MVP Deadline

**Target launch:** August 15, 2025  
We are entering production mode with this PoC to meet our deadline.

---

## 🧱 Architecture Overview

The system is split across 3 coordinated components:

### 1. ⚙️ Solana Program (On-Chain Logic)
Handles:
- Campaign state storage
- Contributor accounts
- Reward pool escrow
- Scoring and reward distribution

### 2. 🧠 Backend Server (Off-Chain GitHub Sync)
Responsibilities:
- Monitor GitHub PRs via the GitHub API
- Score contributors based on a defined model
- Push scores to the Solana program post-deadline

### 3. 🎨 Frontend (Web UI)
Used by:
- Organizers to create and fund campaigns
- Contributors to connect wallets and track contributions
- Public for transparency and campaign visibility

---

## 🧮 Scoring Model

| Contribution Type | Description                        | Score |
|-------------------|------------------------------------|-------|
| ✅ Merged PR      | Successfully merged into main code | 100   |
| 🕵️‍♂️ Unmerged PR with effort | Not merged but reviewed/discussed  | 20    |
| 🚫 Spam/invalid PR| Clearly irrelevant or spammy       | 0     |

The total reward pool is distributed proportionally:

```
reward_i = (score_i / total_scores) * total_reward
```

All scores are normalized across contributors after the scoring sync.

---

## 📜 Instructions Overview (Solana Program)

1. `initialize_campaign` – Create new campaign with USDC pool, timeline, etc.
2. `register_contributor` – Register contributor wallet tied to a GitHub ID.
3. `update_score` – (Called by backend) Update scores after evaluating PRs.
4. `close_campaign` – Mark campaign as ended and ready for reward distribution.
5. `claim_reward` – Allow contributor to withdraw their allocated share from escrow.

---

## 🗂️ On-Chain State Accounts

### `CampaignState`
Stores global campaign info:
- Authority (organizer)
- Campaign name, timeline
- Escrow account
- Total reward pool (USDC)
- GitHub repo metadata
- Total score across all contributors

### `ContributorState`
Stores per-user info:
- Wallet address
- GitHub username (mapped)
- Score (from PR analysis)
- Claimed status
- Reward amount (calculated after score update)

---

## 🔗 Backend GitHub Sync

- Scheduled to run once per campaign, at or after the deadline.
- Pulls all PRs from target repo.
- Classifies and scores based on the scoring model.
- Pushes total contributor scores on-chain via `update_score` instruction.

---

## 🛠️ Technology Stack

- 🧱 **Solana Program:** Anchor (Rust)
- ⚙️ **Backend:** Node.js / Express or Python (TBD)
- 🎨 **Frontend:** React + TailwindCSS
- 💳 **Token:** USDC on Solana

---

## 📅 Milestone Plan

| Date       | Goal                            |
|------------|---------------------------------|
| Aug 7–10   | Finalize Solana smart contract  |
| Aug 11–12  | Backend GitHub sync integration |
| Aug 13–14  | Frontend + end-to-end testing   |
| **Aug 15** | 🎉 PoC MVP launch                |

---

## 🤝 Contributors

Lead Dev: [Your Name]  
GitHub: [github.com/yourusername]  
X (Twitter): [@yourhandle]

---

## ⚠️ Disclaimer

This PoC is designed for experimental and demonstration purposes. Security, scalability, and multi-campaign handling are deferred for future versions.
