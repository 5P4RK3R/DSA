//search List 
export const searchFilter = (list, value)=> {
    return list.filter(x => new RegExp(value, "i").test(Object.values(x)));
  }