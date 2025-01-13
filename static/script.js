const buildStatusInnerHTML = (status) => {
    let icon, text;
    if (status === "active") {
        icon = "material-symbols:check-circle";
        text = "Operational";
    } else if (status === "limited") {
        icon = "material-symbols:offline-bolt";
        text = "Limited";
    } else {
        icon = "material-symbols:error";
        text = "Offline";
    }

    return `
        <iconify-icon class="icon" icon="${icon}" width="22" height="22"></iconify-icon>
        <span class="subheading">${text}</span>
    `;
};

const buildProjectStatusElement = async (projectName, projectLink, requestStatus) => {
    const projectStatusEl = document.createElement("a");
    projectStatusEl.classList.add("project");
    if (projectLink) {
        projectStatusEl.href = projectLink;
        projectStatusEl.target = "_blank";
        projectStatusEl.rel = "noopener noreferrer";
    }

    const loadingIcon = "fluent:arrow-sync-circle-24-filled";

    projectStatusEl.innerHTML = `
        <p class="subheading name">${projectName}</p>
        <p class="status">
            <iconify-icon class="icon" icon="${loadingIcon}" width="22" height="22"></iconify-icon>
            <span class="subheading">Loading</span>
        </p>
    `;

    const statusesContainer = document.getElementById(
        "project-statuses-container",
    );
    statusesContainer.appendChild(projectStatusEl);

    requestStatus().then((status) => {
        const statusInfoP = projectStatusEl.querySelector(".status");
        statusInfoP.innerHTML = buildStatusInnerHTML(status);
        projectStatusEl.classList.add(status);
    });
};


const defaultPingBase = async (endpoint) => {
    try {
        const response = await fetch(endpoint);

        if (response.ok) {
            return "active";
        }
    } catch { }
    return "offline";
};


const getIsleStatsStatus = async () => await defaultPingBase("https://islestats.net/api/ping");
const getStatalyticsStatus = async () => await defaultPingBase("https://statalytics.net/ping");
const getEnotifyStatus = async () => await defaultPingBase("https://enotify.lucism.dev/ping");

document.addEventListener("DOMContentLoaded", async () => {
    buildProjectStatusElement("Statalytics", "https://statalytics.net/", getStatalyticsStatus);
    buildProjectStatusElement("IsleStats", "https://islestats.net/", getIsleStatsStatus);
    buildProjectStatusElement("Enotify", "https://enotify.lucism.dev/", getEnotifyStatus);
});
