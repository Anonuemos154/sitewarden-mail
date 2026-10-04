export type Severity = "info" | "low" | "medium" | "high" | "critical";
export type NavKey = "inbox" | "security" | "cleanup" | "quarantine" | "rules" | "productivity" | "accounts" | "settings";

export interface RiskSignal {
  code: string;
  severity: Severity;
  title: string;
  explanation: string;
}

export interface MailItem {
  id: string;
  sender: string;
  senderAddress: string;
  subject: string;
  preview: string;
  received: string;
  unread: boolean;
  account: string;
  riskScore: number;
  signals: RiskSignal[];
  safeText: string;
  links: Array<{ label: string; destination: string; warning?: string }>;
  quarantined?: boolean;
}
