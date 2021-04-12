import { run } from '../Cargo.toml'
import Chart from 'chart.js';

// Start the show
run()

const opts = { responsive: true, aspectRatio: 1.33, legend: {display: true, maxHeight: 500, position: "bottom", labels: {fontColor: "white", fontStyle: "bold"}}};
const bar_opts = { responsive: true, aspectRatio: 1.33, legend: {display: false }, scales: {xAxes: [{ticks: {fontColor: "white",fontStyle: "bold"}}]}};

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

let tracks, formats, color_spaces, audio_codecs, video_codecs, audio_bitdepths, 
    video_bitdepths, video_standards, chroma_subsamplings, file_extensions, dimensions,
    other_data;


let buildChart = function(name, label, data, opts) {
  new Chart(document.getElementById(name).getContext('2d'), {
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

  if (tracks) {
    tracks.destroy();
    formats.destroy();
    color_spaces.destroy();
    audio_codecs.destroy();
    video_codecs.destroy();
    audio_bitdepths.destroy();
    video_bitdepths.destroy();
    video_standards.destroy();
    chroma_subsamplings.destroy();
    file_extensions.destroy();
    dimensions.destroy();
    other_data.destroy();

  }

  // tracks 
  buildChart("tracks", "# of Tracks per file", JSON.parse(document.getElementById("chart_tracks").textContent), opts)
  buildChart("formats", "Formats", JSON.parse(document.getElementById("chart_formats").textContent), opts)
  buildChart("color_spaces", "Colorspaces", JSON.parse(document.getElementById("chart_color_spaces").textContent), opts)
  buildChart("dimensions", "Dimensions", JSON.parse(document.getElementById("chart_dimensions").textContent), opts)
  buildChart("audio_codecs", "Audio codecs", JSON.parse(document.getElementById("chart_audio_codecs").textContent), opts)
  buildChart("video_codecs", "Video codecs", JSON.parse(document.getElementById("chart_video_codecs").textContent), opts)
  buildChart("audio_bitdepths", "Audio bit depths", JSON.parse(document.getElementById("chart_audio_bitdepths").textContent), opts)
  buildChart("video_bitdepths", "Video bit depths", JSON.parse(document.getElementById("chart_video_bitdepths").textContent), opts)
  buildChart("video_standards", "Video standards", JSON.parse(document.getElementById("chart_video_standards").textContent), bar_opts)
  buildChart("chroma_subsamplings", "Chroma subsamplings", JSON.parse(document.getElementById("chart_chroma_subsamplings").textContent), opts)
  buildChart("file_extensions", "File extensions", JSON.parse(document.getElementById("chart_file_extensions").textContent), opts)
  buildChart("other_data", "Other data", JSON.parse(document.getElementById("chart_other_data").textContent), opts)

};


