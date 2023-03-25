document.addEventListener("DOMContentLoaded", () => {
  loadData();

  const form = document.getElementById("text-form");
  form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const input = document.getElementById("text-input");
    const content = input.value;
    const data = { content };

    try {
      const response = await fetch("/submit_data", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (response.status === 201) {
        console.log("Data submitted successfully");
        input.value = ""; // Clear the input field
        loadData(); // Refresh the data on the page
      } else {
        console.error("Failed to submit data");
      }
    } catch (error) {
      console.error("Error submitting data:", error);
    }
  });
});

async function loadData() {
  const response = await fetch("/get_data");
  const data = await response.json();

  const container = document.getElementById("texts-container");
  container.innerHTML = "";

  data.forEach((text) => {
    const div = document.createElement("div");
    div.textContent = `ID: ${text.id}, Content: ${text.content}`;
    container.appendChild(div);
  });
}
