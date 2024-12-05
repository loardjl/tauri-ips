import { onUnmounted } from 'vue'
import { useMenuStore } from '@src/store/useMenu'
import { invoke } from '@tauri-apps/api/tauri'

const URL = await invoke('get_api_host')
export const useApi = () => {
  const menuStore = useMenuStore()
  const fetchGetApi = async url => {
    try {
      const res = await invoke('send_http_get_msg', {
        url: url ?? URL
      })
      return res
    } catch (error) {
      return Promise.reject(error)
    }
  }
  const fetchPostApi = async (data, type = 'datacenter', url) => {
    try {
      const res = await invoke('send_http_post_msg', {
        url: url ?? URL + '/' + type,
        data
      })
      return JSON.parse(res)
    } catch (error) {
      return Promise.reject(error)
    }
  }
  onUnmounted(() => {
    menuStore.clearAsideList()
  })
  return { fetchGetApi, fetchPostApi }
}
