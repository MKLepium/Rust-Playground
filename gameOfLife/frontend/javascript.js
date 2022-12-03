var xhttp = new XMLHttpRequest();

var initialized = false;
var started = false;
var generations = 0;
var liveCells = 0;
var m_width = 0;
var m_height = 0;
var last_generations_array = [];

function init() {
    "use strict";
    newBoard();
}

function newBoard() {
    "use strict";
    started = false;
    document.getElementById("startButton").innerHTML = "Start";
    let height = prompt("Please enter your wished height", "50");
    let width = prompt("Please enter your wished width", "50");
    m_height = height;
    m_width = width;
    last_generations_array = new Array(parseInt(m_width));
    for (var i = 0; i < last_generations_array.length; i++) {
        last_generations_array[i] = new Array(parseInt(m_height)).fill("0");
    }
    if (height != null && width != null) {
        xhttp.onreadystatechange = recieve_JSON;
        xhttp.open("GET", "NewBoard?height=" + height + "&width=" + width);
        xhttp.send();
    }
}

function reset() {
    "use strict";
    started = false;
    document.getElementById("startButton").innerHTML = "Start";
    xhttp.onreadystatechange = recieve_JSON;
    xhttp.open("GET", "ResetBoard");
    xhttp.send();
}

function Advance() {
    "use strict";
    started = false;
    document.getElementById("startButton").innerHTML = "Start";
    xhttp.onreadystatechange = recieve_JSON;
    xhttp.open("GET", "Advance");
    xhttp.send();
}

function randomize() {
    "use strict";
    started = false;
    generations = 0;
    document.getElementById("startButton").innerHTML = "Start";
    xhttp.onreadystatechange = recieve_JSON;
    xhttp.open("GET", "Randomize");
    xhttp.send();
}

async function start() {
    "use strict";
    if (document.getElementById("startButton").innerHTML === "Start") {
        document.getElementById("startButton").innerHTML = "Stop";
        started = true;
        while (started === true) {
            xhttp.onreadystatechange = recieve_JSON;
            xhttp.open("GET", "Advance");
            xhttp.send();
            await new Promise(r => setTimeout(r, 750));
        }
    } else {
        document.getElementById("startButton").innerHTML = "Start";
        started = false;
    }
}

function recieve_JSON() {
    "use strict";
    if (xhttp.readyState == 4 && xhttp.status == 200) {
        //document.getElementById("game").textContent = xhttp.responseText;
        var json = JSON.parse(xhttp.responseText);
        m_height = parseInt(json.height);
        m_width = parseInt(json.width);
        //console.log(json);
        generations = parseInt(json.generations);
        liveCells = parseInt(json.live_cells);
        document.getElementById("generations").innerHTML = "Generations: " + generations;
        document.getElementById("liveCells").innerHTML = "Live Cells: " + liveCells;
        //console.log(parseInt(json.GameOfLife.width));
        var array = new Array(parseInt(json.width));
        for (var i = 0; i < array.length; i++) {
            array[i] = new Array(parseInt(json.height));
        }
        var j = 0;
        for (var x = 0; x < array.length; x++) {
            for (var y = 0; y < array[x].length; y++) {
                array[x][y] = json.array[j];
                if (array[x][y] == "1") {
                    last_generations_array[x][y] = json.array[j];
                }
                j++;
            }
        }
        //console.log(array);
        generate_table(array);
    }
}

function swap_data(id) {
    "use strict";
    var cell = document.getElementById(id);
    if (cell.classList.contains("on")) {
        cell.classList.remove("on");
        cell.classList.add("activatedbefore");
    } else if (cell.classList.contains("off")) {
        cell.classList.remove("off");
        cell.classList.add("on");
    } else {
        cell.classList.remove("activatedbefore");
        cell.classList.add("on");
    }
    xhttp.onreadystatechange = state_change;
    xhttp.open("GET", "ChangeValue?id=" + id);
    xhttp.send();

}

function state_change() {
    "use strict";
    if (xhttp.readyState == 4 && xhttp.status == 200) {
        var json = JSON.parse(xhttp.responseText);
        liveCells = parseInt(json.live_cells);
        document.getElementById("liveCells").innerHTML = "Live Cells: " + liveCells;
        // NOthing needs to be done 
        // Maybe if the server crashes
    }
}

function generate_table(array) {
    "use strict";
    var div = document.getElementById("game");
    // IF table already exists no need to create
    var potential_table = document.getElementById("Table");
    if (potential_table != null) {
        potential_table.remove();
    }
    var table = document.createElement("table");
    table.id = "Table";

    // Creating cells

    var x = 0;
    for (var i = 0; i < m_width; i++) {
        var row = document.createElement("tr");
        for (var j = 0; j < m_height; j++) {
            var cell = document.createElement("td");
            cell.id = x;

            if (array[i][j] == "0") {
                if (last_generations_array[i][j] == "1") {
                    cell.classList.add("activatedbefore");
                } else {
                    cell.classList.add("off");
                }
            } else {
                cell.classList.add("on");
            }
            cell.setAttribute("onclick", "swap_data(" + x + ")");

            x = x + 1;

            row.appendChild(cell);
        }
        table.appendChild(row);
    }
    div.appendChild(table);
}