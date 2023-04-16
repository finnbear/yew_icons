function waitUntilVisible(selector) {
  return new Promise((resolve) => {
    if (document.querySelector(selector)) {
      return resolve(document.querySelector(selector));
    }

    const observer = new MutationObserver(() => {
      if (document.querySelector(selector)) {
        resolve(document.querySelector(selector));
        observer.disconnect();
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true,
    });
  });
}

function showLoading(show) {
  const loadingEl = document.querySelector("#loading");

  if (show) {
    loadingEl.classList.add("show");
    console.log("🕗 Loading wasm...");
  } else {
    loadingEl.classList.remove("show");
  }
}

async function main() {
  let visible = false;
  let isLoading = false;

  // If after 1s the wasm has no loaded, we show a loading indicator
  setTimeout(() => {
    if (!visible) {
      showLoading(true);
      isLoading = true;
    }
  }, 1000);

  // We wait until the gallery is visible and show a loading indicator
  waitUntilVisible(".gallery").then(() => {
    visible = true;
    console.log("✅ wasm loaded");
  });

  if (!isLoading) {
    return;
  }

  // Remove loading indicator
  showLoading(false);
  isLoading = false;
}

main().catch(console.error);
