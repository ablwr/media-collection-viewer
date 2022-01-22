import { run } from '../Cargo.toml'
import Chart from 'chart.js';

// Start the show
run()

const opts: {} = { responsive: true, aspectRatio: 1.33, legend: {display: true, maxHeight: 500, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}};
const bar_opts: {} = { responsive: true, aspectRatio: 1.33, legend: {display: false }, scales: {xAxes: [{ticks: {fontColor: "white",fontStyle: "bold"}}]}};

const randCols = [
    "#25CCF7","#FD7272","#54a0ff","#00d2d3",
    "#1abc9c","#2ecc71","#3498db","#9b59b6","#34495e",
    "#16a085","#27ae60","#2980b9","#8e44ad","#2c3e50",
    "#f1c40f","#e67e22","#e74c3c","#ecf0f1","#95a5a6",
    "#f39c12","#d35400","#c0392b","#bdc3c7","#7f8c8d",
    "#55efc4","#81ecec","#74b9ff","#a29bfe","#dfe6e9",
    "#00b894","#00cec9","#0984e3","#6c5ce7","#ffeaa7",
    "#fab1a0","#ff7675","#fd79a8","#fdcb6e","#e17055",
    "#d63031","#feca57","#5f27cd","#54a0ff","#01a3a4"
]

function buildChart({ name, label, data, opts }: { name: string; label: string; data: {}; opts: {}; }): void {
  const ctx = document.getElementById(name).getContext('2d');
  return new Chart(ctx, {
    type: 'pie',
    data: {
      labels: Object.keys(data),
      datasets: [{
          label: label,
          data: Object.values(data),
          backgroundColor: randCols,
          borderWidth: 0,
      }]},
      options: opts,    
  });
}

document.getElementById('jsonStart').onclick = function() {
  buildChart({ name: "tracks", label: "# of Tracks per file", data: JSON.parse(document.getElementById("chart_tracks").textContent), opts })
  buildChart({ name: "formats", label: "Formats", data: JSON.parse(document.getElementById("chart_formats").textContent), opts })
  buildChart({ name: "color_spaces", label: "Colorspaces", data: JSON.parse(document.getElementById("chart_color_spaces").textContent), opts })
  buildChart({ name: "dimensions", label: "Dimensions", data: JSON.parse(document.getElementById("chart_dimensions").textContent), opts })
  buildChart({ name: "audio_codecs", label: "Audio codecs", data: JSON.parse(document.getElementById("chart_audio_codecs").textContent), opts })
  buildChart({ name: "video_codecs", label: "Video codecs", data: JSON.parse(document.getElementById("chart_video_codecs").textContent), opts })
  buildChart({ name: "audio_bitdepths", label: "Audio bit depths", data: JSON.parse(document.getElementById("chart_audio_bitdepths").textContent), opts })
  buildChart({ name: "video_bitdepths", label: "Video bit depths", data: JSON.parse(document.getElementById("chart_video_bitdepths").textContent), opts })
  buildChart({ name: "video_standards", label: "Video standards", data: JSON.parse(document.getElementById("chart_video_standards").textContent), opts: bar_opts })
  buildChart({ name: "chroma_subsamplings", label: "Chroma subsamplings", data: JSON.parse(document.getElementById("chart_chroma_subsamplings").textContent), opts })
  buildChart({ name: "file_extensions", label: "File extensions", data: JSON.parse(document.getElementById("chart_file_extensions").textContent), opts })
  buildChart({ name: "other_data", label: "Other data", data: JSON.parse(document.getElementById("chart_other_data").textContent), opts })
};


