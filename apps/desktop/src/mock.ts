import type { MailItem } from "./types";

export const demoMessages: MailItem[] = [
  {
    id: "m-001",
    sender: "MSD Systems",
    senderAddress: "updates@example.invalid",
    subject: "Development build report",
    preview: "CI completed for the current local alpha build…",
    received: "12:41",
    unread: true,
    account: "work@example.invalid",
    riskScore: 4,
    signals: [{ code: "auth.dmarc.pass", severity: "info", title: "Authentication signal present", explanation: "Synthetic demo message; provider authentication would be shown here." }],
    safeText: "Development build report\n\nThe current alpha build completed its synthetic test suite. This demonstration contains no real mailbox data.",
    links: []
  },
  {
    id: "m-002",
    sender: "Accounts Team",
    senderAddress: "billing@accounts-example.invalid",
    subject: "Action required: payment profile",
    preview: "Please review the updated payment destination…",
    received: "11:18",
    unread: true,
    account: "work@example.invalid",
    riskScore: 72,
    signals: [
      { code: "identity.reply_to_mismatch", severity: "high", title: "Reply-To domain differs", explanation: "Replies would be sent to a domain different from the visible sender domain." },
      { code: "url.visible_target_mismatch", severity: "medium", title: "Displayed URL differs from target", explanation: "The visible link label and parsed destination do not match." }
    ],
    safeText: "Action required: payment profile\n\nPlease review the updated payment destination.\n\nSafe View does not activate the original link.",
    links: [{ label: "https://accounts-example.invalid", destination: "https://login-example.invalid/session", warning: "Visible label and destination differ" }]
  },
  {
    id: "m-003",
    sender: "Newsletter",
    senderAddress: "news@updates-example.invalid",
    subject: "Weekly product digest",
    preview: "A long-running newsletter candidate for cleanup…",
    received: "Yesterday",
    unread: false,
    account: "personal@example.invalid",
    riskScore: 8,
    signals: [],
    safeText: "Weekly product digest\n\nSynthetic newsletter content for the cleanup demonstration.",
    links: [{ label: "View online", destination: "https://updates-example.invalid/news" }]
  },
  {
    id: "m-004",
    sender: "Unknown sender",
    senderAddress: "document@unknown-example.invalid",
    subject: "Invoice attachment",
    preview: "Attachment held for analysis…",
    received: "Yesterday",
    unread: false,
    account: "work@example.invalid",
    riskScore: 91,
    quarantined: true,
    signals: [
      { code: "attachment.executable", severity: "critical", title: "Executable attachment indicator", explanation: "The synthetic attachment type is blocked from automatic opening." }
    ],
    safeText: "Invoice attachment\n\nAttachment content is not opened in the trusted view.",
    links: []
  }
];
