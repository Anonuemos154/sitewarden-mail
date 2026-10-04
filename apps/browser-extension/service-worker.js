chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "sitewarden-open",
    title: "In SiteWarden Mail prüfen",
    contexts: ["page", "selection", "link"]
  });
});

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  // Foundation only:
  // Native host protocol must be versioned and must never accept arbitrary shell commands.
  const payload = {
    protocolVersion: 1,
    action: "inspect_context",
    tabUrl: tab?.url ?? null,
    selectedText: info.selectionText ?? null,
    linkUrl: info.linkUrl ?? null
  };

  try {
    const response = await chrome.runtime.sendNativeMessage(
      "de.sitewarden.mail.bridge",
      payload
    );
    console.log("SiteWarden Mail response", response?.status ?? "unknown");
  } catch (e) {
    console.warn("SiteWarden Mail desktop bridge unavailable");
  }
});
