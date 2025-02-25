async function fetchChain() {
    const response = await fetch("go env");
    const data = await response.json();
    document.getElementById("chain").innerText = JSON.stringify(data, null, 2);
}
