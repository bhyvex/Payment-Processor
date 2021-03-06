'use strict';

angular.module('paymentProcessor')
  .controller('IncomesCtrl', incomesCtrl);

function incomesCtrl ($q, loginManager, transactionsManager) {
  const viewModel = this;

  /** Controller Variables **/
  viewModel.transactions = [];
  viewModel.user = null;

  /** Controller Functions **/


  _initController();

  /******** Implementation *******/

  function _initController () {
    const _setIncomes = (values) => {
      viewModel.user = values[0];
      viewModel.transactions = transactionsManager.thatAreIncomes(values[0].id, values[1]);
    };

    $q.all([
      loginManager.getUser(),
      transactionsManager.getAll()
    ]).then(_setIncomes);
  }
}
