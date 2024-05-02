//get Pagination
export const getPagination = (currentPage, itemsPerPage, list)=> {
    let indexOfLastTodo = currentPage * itemsPerPage;
    let indexOfFirstTodo = indexOfLastTodo - itemsPerPage;
    let lists = list.slice(indexOfFirstTodo, indexOfLastTodo);
    let pageNumbers = [];
    for (let i = 1; i <= Math.ceil(list.length / itemsPerPage); i++) {
      pageNumbers = [...pageNumbers, i];
    }
    
    return [lists, pageNumbers];
  }