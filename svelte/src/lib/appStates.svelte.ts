class LinksState {
  isAnyBrokenLinks = $state(false);
  isAnyMeteToRefresh = $state(false);
}

export const linksState = new LinksState();
