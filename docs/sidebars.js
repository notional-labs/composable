/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

 const isProd = process.env.NODE_ENV === "production";

 // @ts-check
 
 /** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
 const sidebars = {
   // By default, Docusaurus generates a sidebar from the docs folder structure
   // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],
 
   // But you can create a sidebar manually
   internalSidebar: [{ type: "autogenerated", dirName: "internal" }],
   user_guides: [
     {
       type: "category",
       label: "User Guides",
       link: {
         type: "generated-index",
         slug: "user-guides",
       },
       collapsible: false,
       items: [
         {
           type: "category",
           label: "Accounts and Wallets",
           link: {
             type: "generated-index",
             slug: "accounts-wallets",
           },
           collapsible: true,
           items: [
             "user-guides/polkadotjs-extension-create-account",
             "user-guides/talisman-create-account",
             "user-guides/keplr-guide",
             "user-guides/layr-guide",
           ],
         },
         {
          type: "category",
          label: "Solana Restaking",
          link: {
            type: "generated-index",
            slug: "solana-restaking",
          },
          collapsible: true,
          items: [
            "user-guides/restaking-sol",
            "user-guides/team-competition",
          ],
        },
         {
           type: "category",
           label: "Transactions and Trading",
           link: {
             type: "generated-index",
             slug: "transactions-and-trading",
           },
           collapsible: true,
           items: [
             "user-guides/composable-cosmos-staking",
             "user-guides/how-to-provide-liquidity",
             "user-guides/how-to-trade-pica-on-pablo",
             "user-guides/dot-lp-guide",
             "user-guides/trustless-transfers",
           ],
         },
         "user-guides/picasso-governance",
       ],
     },
      {
      type: "category",
      label: "Developer Guides",
      link: {
        type: "generated-index",
        slug: "developer-guides",
      },
      collapsible: false,
      items: [
        "develop/composable-cosmos",
        "develop/solana-avs-testnet",
        "develop/local-picasso-guide",
        "develop/oracle-set-up-guide",
        "develop/collator-guide",
        "develop/nix",
        "develop/codespaces",

      ],
    },
   ],
   ibc: [
    {
    type: "category",
       label: "Inter-Blockchain Communication Protocol (IBC)",
       link: {
         type: "doc",
         id: "technology/ibc",
       },
       collapsible: false,
       collapsed: false,
       items: [
        "technology/ibc/ethereum",
        "technology/ibc/solana",
        "technology/ibc/polkadot",
        "technology/ibc/polkadot-kusama",
        "technology/ibc/near",
        "technology/ibc/hyperspace-relayer",
        "technology/ibc/light-clients",
        "technology/ibc/merkle-mountain-ranges",
        "technology/ibc/beefy-light-client",
       ],
      },

],

concepts: [
 
  "concepts/picasso",
  "concepts/ibc",
  "concepts/restaking",
  "concepts/definitions",

 
],

token: [
 
      "governance-&-token/use-cases",
      "governance-&-token/tokenomics",
      "governance-&-token/governance",
      "governance-&-token/token-transparency",
  
],

restaking: [
{
  type: "category",
      label: "Generalized Restaking",
      link: {
       type: "doc",
        id: "technology/restaking",
     },
    collapsible: false,
    collapsed: false,
    items: [
  "technology/restaking/architecture",
  "technology/restaking/governance",
  "technology/restaking/use-cases",
  "technology/restaking/sol-ibc-avs",
  "technology/restaking/vaults",
  "technology/restaking/mantis-games",
  "technology/restaking/roadmap"
   ],
},

],

mantis: [
  {
    type: "doc",
    id: "technology/mantis",
  },
   "technology/mantis/benefits-use-cases",
   "technology/mantis/protocol-architecture",
   "technology/mantis/protocol-flow",
   "technology/mantis/solvers-solutions",
   "technology/mantis/solver-integration",
   "technology/mantis/solver-guide",
   "technology/mantis/integration-guide",
   "technology/mantis/tools",
   {
    type: "category",
        label: "CVM",
        link: {
         type: "doc",
         id: "technology/cvm",
       },
      collapsible: true,
      collapsed: true,
      items: [
      "technology/cvm/tutorial",
 ],
},

],

 };
 
 // if (!isProd) {
 //     sidebars.tutorialSidebar.unshift({
 //         type: 'category',
 //         label: 'test-SCDI',
 //         link: {
 //             type: 'doc',
 //             id: 'testSCDI/entry',
 //         },
 //         collapsible: true,
 //         collapsed: true,
 //         items: [
 //             {
 //                 type: 'link',
 //                 label: 'test-SCDI',
 //                 href: '/test-vm',
 //             },
 //         ],
 //     });
 // }
 
 module.exports = sidebars;