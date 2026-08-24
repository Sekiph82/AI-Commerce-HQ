import { invoke } from '@tauri-apps/api/core';

export type GitHealth = 'CLEAN' | 'DIRTY' | 'CONFLICTED' | 'DETACHED' | 'UNBORN' | 'MISSING' | 'NON_GIT';
export type GitFileChange = { path: string; kind: string; staged: boolean; unstaged: boolean };
export type GitCommit = { sha: string; subject: string; authorName: string; authorEmail: string; authoredAt: string; committedAt: string; parentCount: number };
export type GitRemote = { name: string; fetchUrl: string; pushUrl: string | null };
export type GitWorktree = { path: string; branch: string | null; headSha: string | null; locked: boolean; prunable: boolean };
export type GitSnapshot = {
  projectId: string; repositoryId: string; repositoryPath: string; currentBranch: string | null; detachedHead: boolean; headSha: string | null;
  stagedFiles: GitFileChange[]; unstagedFiles: GitFileChange[]; untrackedFiles: string[]; conflictedFiles: string[];
  aheadCount: number | null; behindCount: number | null; upstream: string | null; remotes: GitRemote[]; recentCommits: GitCommit[]; worktrees: GitWorktree[]; health: GitHealth; snapshotTimestamp: string;
};

export function getGitSnapshot(projectId: string, persist = false) { return invoke<GitSnapshot>('hiveai_git_snapshot', { request: { projectId, persist } }); }
