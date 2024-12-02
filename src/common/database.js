import localforage from 'localforage'
localforage.config({
  driver: localforage.INDEXEDDB,
  name: 'UJ-DB',
  version: 1.0,
  storeName: 'UJ-Store',
  description: 'UJ-Store'
})
localforage.ready().then(() => {
  console.log('LocalForage is ready')
})

export { localforage }
