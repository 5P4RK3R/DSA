export const getUniqueObjects = (json) =>{
    let jsonObject = json.map(JSON.stringify);
    uniqueSet = new Set(jsonObject); 
    return uniqueArray = Array.from(uniqueSet).map(JSON.parse);
}