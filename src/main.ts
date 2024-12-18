import { invoke } from "@tauri-apps/api/core";

let inputEl			: HTMLInputElement | null = null;
let forecastEl	: HTMLElement | null 			= null;

async function generateForecast() {
	if (forecastEl && inputEl) {
		const city = inputEl.value;
		try {
			const response: string = await invoke("forecast", { city });
			forecastEl.innerHTML = formatWeatherData(response);
		} catch (error) {
			forecastEl.textContent = `Error: ${error}`;
		}
	}
}

function formatWeatherData(data: string): string {
	const dataLines = data.split("\n");
	console.log(dataLines);
	
	const location = dataLines[0];
	const temperature = dataLines[1];
	const condition = dataLines[2];
	
	return `
		<h2>Weather Information</h2>
		<div class="forecast-item">
			<li><strong>Location:</strong> ${location}</li>
		</div>
		<div class="forecast-item">
			<li><strong>Temperature:</strong> ${temperature}°C</li>
		</div>
		<div class="forecast-item">
			<li><strong>Condition:</strong> ${condition}</li>
		</div>
	`;
}

window.addEventListener("DOMContentLoaded", () => {
	inputEl = document.querySelector("#city-input");
	forecastEl = document.querySelector("#forecast");

	document.querySelector("#city-form")?.addEventListener("submit", (e) => {
		e.preventDefault();
		generateForecast();
	});
});
