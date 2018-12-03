import { connect } from 'react-redux'
import App from './App'
import {
  getMyHash
} from './actions'

const mapStateToProps = state => {
  return {}
}

const mapDispatchToProps = (dispatch) => {
  return {
    getMyHash: () => {
      dispatch(getMyHash())
    }
  }
}

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(App)
